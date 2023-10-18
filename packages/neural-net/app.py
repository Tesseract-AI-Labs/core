import os
import numpy as np
from flask import Flask, request, render_template
import pandas as pd 
import scipy
from PIL import Image
import time
import hashlib

# torch.
import torch
import torch.nn as nn
from torch.utils.data import Dataset,DataLoader
from torch.utils.data.sampler import SequentialSampler, RandomSampler
from torch.utils.data import DataLoader, Dataset
from torchvision.models.detection.faster_rcnn import FastRCNNPredictor
from torchvision.models.detection import FasterRCNN
from torchvision.models.detection.rpn import AnchorGenerator
from torchvision.transforms import ToTensor

import torch.nn.functional as F

import pydicom
from pydicom.pixel_data_handlers.util import apply_voi_lut

import matplotlib.pyplot as plt


app = Flask(__name__)

# COCO classes
CLASSES = [
    'Aortic enlargement', 'Atelectasis', 'Calcification', 'Cardiomegaly', 'Consolidation',
    'ILD', 'Infiltration', 'Lung Opacity', 'Nodule/Mass', 'Other lesion', 
    'Pleural effusion', 'Pleural thickening', 'Pneumothorax', 'Pulmonary fibrosis', 'No Finding'
]

# colors for visualization
COLORS = [[0.000, 0.447, 0.741], [0.850, 0.325, 0.098], [0.929, 0.694, 0.125],
          [0.494, 0.184, 0.556], [0.466, 0.674, 0.188], [0.301, 0.745, 0.933]]

class DETRModel(nn.Module):
    def __init__(self,num_classes,num_queries):
        super(DETRModel,self).__init__()
        self.num_classes = num_classes
        self.num_queries = num_queries
        self.model = torch.hub.load('facebookresearch/detr', 'detr_resnet50', pretrained=True)
        
        for param in self.model.parameters():
            param.requires_grad = True


        self.in_features = self.model.class_embed.in_features
        
        self.model.class_embed = nn.Linear(in_features=self.in_features,out_features=self.num_classes+1)
        self.model.num_queries = self.num_queries
        
    def forward(self,images):
        return self.model(images)
    
def load_model():
    ## Loading a model
    num_classes = 15
    num_queries = 2
    model = DETRModel(num_classes=num_classes,num_queries=num_queries)
    model.load_state_dict(torch.load("./models/detr_model.pth", map_location=torch.device('cpu')))
    return model

# for output bounding box post-processing
def box_cxcywh_to_xyxy(x):
    x_c, y_c, w, h = x.unbind(1)
    b = [(x_c - 0.5 * w), (y_c - 0.5 * h),
         (x_c + 0.5 * w), (y_c + 0.5 * h)]
    return torch.stack(b, dim=1)

def rescale_bboxes(out_bbox, size):
    img_w, img_h = size
    b = box_cxcywh_to_xyxy(out_bbox)
    b = b * torch.tensor([img_w, img_h, img_w, img_h], dtype=torch.float32)
    return b

def read_xray(path, voi_lut = True, fix_monochrome = True):
    dicom = pydicom.read_file(path)

    # VOI LUT (if available by DICOM device) is used to transform raw DICOM data to "human-friendly" view
    if voi_lut:
        data = apply_voi_lut(dicom.pixel_array, dicom)
    else:
        data = dicom.pixel_array

    # depending on this value, X-ray may look inverted - fix that:
    if fix_monochrome and dicom.PhotometricInterpretation == "MONOCHROME1":
        data = np.amax(data) - data

    data = data - np.min(data)
    data = data / np.max(data)
    data = (data * 255).astype(np.uint8)
    data = np.stack([data]*3).transpose(1,2,0)
    return data

def dicom_to_pil(dicom):
    data = dicom.pixel_array
    data = data - np.min(data)
    data = data / np.max(data)
    data = (data * 255).astype(np.uint8)
    image = Image.fromarray(data)
    return image.convert('RGB')


@app.route('/')
def home():
    return render_template('index.html')

@app.route('/predict', methods=['POST', 'GET'])
def predict():
    device = torch.device('cpu')

    model = load_model()
    model.eval()
    model.to(device)

    # DIR = "./models/test_image1.dicom"

    if 'file' not in request.files:
        return 'No file part', 400
    file = request.files['file']
    if file.filename == '':
        return 'No selected file', 400

    dicom = pydicom.dcmread(file)
    pil_img = dicom_to_pil(dicom)
    
    transform = ToTensor()
    image_tensor = transform(pil_img).unsqueeze(0)  # Add batch dimension

    index_to_show = 0
    _, h, w = image_tensor[index_to_show].shape  # For denormalizing images
    print("h, w:", h, w)

    images = [img.to(device) for img in image_tensor]

    with torch.no_grad():
        outputs = model(images)

    probas = outputs['pred_logits'].softmax(-1)[index_to_show, :, :-1]
    keep = probas.max(-1).values > 0.08
    
    boxes = rescale_bboxes(outputs['pred_boxes'][index_to_show, keep], (512, 512))
    prob = probas[keep]
    
    # Visualization of Predicted Boxes
    plt.figure(figsize=(16, 10))
    plt.imshow(pil_img)
    ax = plt.gca()
    colors = COLORS * 100
    for p, (xmin, ymin, xmax, ymax), c in zip(prob, boxes.tolist(), colors):
        ax.add_patch(plt.Rectangle((xmin, ymin), xmax - xmin, ymax - ymin,
                                   fill=False, color=c, linewidth=3))
        cl = p.argmax()
        text = f'{CLASSES[cl]}: {p[cl]:0.2f}'
        ax.text(xmin, ymin, text, fontsize=15,
                bbox=dict(facecolor='yellow', alpha=0.5))
    plt.axis('off')

    # Save the plot as an image
    result_image = 'static/result.png'
    plt.savefig(result_image, bbox_inches='tight') 

    plt.show()

    # Generate a unique ID for this prediction based on the current time
    timestamp = int(time.time() * 1000)  # milliseconds since epoch

    # Convert timestamp to string before hashing
    timestamp_str = str(timestamp)

    # Create a hash of the timestamp
    prediction_id = hashlib.sha256(timestamp_str.encode()).hexdigest()

    print(prediction_id)
    return render_template('index.html', result_image_url=result_image, hash=prediction_id)

if __name__ == "__main__":
    app.run()