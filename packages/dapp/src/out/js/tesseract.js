"use strict";
(() => {
  // src/code/js/cascade-class-name.ts
  async function cascadeClassName(element) {
    const SPEED = 1e3 / 60;
    element.classList.add("Active");
    const children = Array.from(element.childNodes);
    for (const child of children) {
      if (child.nodeType === Node.ELEMENT_NODE) {
        await new Promise((resolve) => setTimeout(resolve, SPEED));
        await cascadeClassName(child);
      } else if (child.nodeType === Node.TEXT_NODE && child.nodeValue?.trim()) {
        const text = child.data;
        const parent = child.parentNode;
        parent.removeChild(child);
        child.textContent = "";
        for (const char of text.split("")) {
          if (char === " ") {
            parent.insertAdjacentHTML("beforeend", " ");
          } else {
            const span = document.createElement("span");
            span.textContent = char;
            span.className = "Active";
            parent.appendChild(span);
          }
          await new Promise((resolve) => setTimeout(resolve, SPEED));
        }
      }
    }
  }

  // src/code/js/forget.ts
  var forget = function(inputs) {
    const targets = inputs;
    if (targets.length) {
      let x = targets.length;
      while (x--) {
        let y = targets[x].children.length;
        while (y--)
          targets[x].children[y].className = "";
      }
    } else {
      let x = targets.children.length;
      while (x--) {
        targets.children[x].className = "";
      }
    }
  };

  // src/code/js/note.ts
  var context;
  if (window.AudioContext) {
    context = new AudioContext();
  } else if (window.webkitAudioContext) {
    context = new webkitAudioContext();
  } else {
    context = {};
  }
  function note(frequency, meta, callback) {
    if (!meta)
      meta = {};
    const undef = void 0;
    if (meta.type == undef)
      meta.type = "sine";
    if (meta.volume == undef)
      meta.volume = 0.03125;
    if (meta.sustain == undef)
      meta.sustain = 0;
    if (meta.chord == undef)
      meta.chord = false;
    if (meta.reverb == undef)
      meta.reverb = 0.25;
    if (typeof frequency !== "number") {
      let x = frequency.length;
      while (x--) {
        if (x)
          note(frequency[x], meta);
        else
          return note(frequency[x], meta, callback);
      }
    }
    const o = context.createOscillator();
    const g = context.createGain();
    o.type = meta.type;
    o.connect(g);
    g.gain.value = meta.volume;
    o.frequency.value = frequency;
    g.connect(context.destination);
    o.start(0);
    if (!meta.chord) {
      g.gain.setTargetAtTime(0, context.currentTime + meta.sustain, meta.reverb);
    } else
      meta.chord.push(
        function(g2, context2, meta2) {
          return function() {
            g2.gain.setTargetAtTime(
              0,
              context2.currentTime + meta2.sustain,
              meta2.reverb
            );
          };
        }(g, context, meta)
      );
    if (callback) {
      if (meta.chord) {
        let x = meta.chord.length;
        while (x--)
          meta.chord[x]();
      }
      callback();
    }
  }

  // src/code/js/logo-click.ts
  var logoClick = async function() {
    const Info = document.getElementById("Info");
    if (!Info)
      return;
    location.hash = "";
    if (logo_state) {
      note(800);
      document.body.className = "";
      history.replaceState({}, document.title, ".");
      forget(Info);
      forget(Info.children);
      forget(Info.getElementsByTagName("h1")[0]);
    } else {
      note(900);
      document.body.className = "Active";
      await cascadeClassName(Info);
    }
    setLogoState(!getLogoState());
    if (window.innerHeight < window.innerWidth) {
      const videos = document.getElementsByTagName("video");
      if (videos) {
        const video = videos[0];
        video.src = "https://storageapi.fleek.co/2e62e11d-d4be-4c6f-a2bb-b159c83a0d95-bucket/ubq.fi/hero.mp4";
        video.addEventListener("play", function loaded() {
          console.log(`playing`);
          video.className += "Active";
        });
      }
    }
  };

  // src/code/js/the-grid.ts
  function grid(node = document.body) {
    const canvas = document.createElement("canvas");
    const devicePixelRatio = window.devicePixelRatio || 1;
    canvas.width = window.innerWidth * devicePixelRatio;
    canvas.height = window.innerHeight * devicePixelRatio;
    node.appendChild(canvas);
    const gl = canvas.getContext("webgl");
    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);
    const vertexShaderSource = `
      attribute vec2 a_position;
  
      void main() {
          gl_Position = vec4(a_position, 0, 1);
      }
  `;
    const fragmentShaderSource = `
      precision mediump float;
  
      uniform vec2 u_resolution;
      uniform float u_time;
  
      float rand(vec2 n) {
          return fract(sin(dot(n, vec2(12.9898, 4.1414))) * 43758.5453);
      }
  
      void main() {
          vec3 color = vec3(128.0/255.0, 128.0/255.0, 128.0/255.0); // #808080
          vec2 tilePosition = mod(gl_FragCoord.xy, 24.0);
          vec2 tileNumber = floor(gl_FragCoord.xy / 24.0);
  
          float period = rand(tileNumber) * 9.0 + 1.0; // Random value in the range [1, 10]
          float phase = fract(u_time / period / 8.0); // Animation eight times slower
          float opacity = (1.0 - abs(phase * 2.0 - 1.0)) * 0.125; // Limit maximum opacity to 0.25
  
          vec4 backgroundColor = vec4(color, opacity);
  
          if (tilePosition.x > 23.0 && tilePosition.y < 1.0) {
            gl_FragColor = vec4(color, 1.0); // Full opacity for the dot
        } else {
            gl_FragColor = backgroundColor;
        }
      }
  `;
    function createShader(gl2, type, source) {
      const shader = gl2.createShader(type);
      gl2.shaderSource(shader, source);
      gl2.compileShader(shader);
      if (!gl2.getShaderParameter(shader, gl2.COMPILE_STATUS)) {
        console.error(
          "An error occurred compiling the shaders: " + gl2.getShaderInfoLog(shader)
        );
        gl2.deleteShader(shader);
        return null;
      }
      return shader;
    }
    const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = createShader(
      gl,
      gl.FRAGMENT_SHADER,
      fragmentShaderSource
    );
    const program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
      console.error(
        "Unable to initialize the shader program: " + gl.getProgramInfoLog(program)
      );
      return;
    }
    gl.useProgram(program);
    const timeUniformLocation = gl.getUniformLocation(program, "u_time");
    const resolutionUniformLocation = gl.getUniformLocation(
      program,
      "u_resolution"
    );
    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]),
      gl.STATIC_DRAW
    );
    const positionAttributeLocation = gl.getAttribLocation(program, "a_position");
    gl.enableVertexAttribArray(positionAttributeLocation);
    gl.vertexAttribPointer(positionAttributeLocation, 2, gl.FLOAT, false, 0, 0);
    function resizeCanvasToDisplaySize(canvas2) {
      const displayWidth = window.innerWidth;
      const displayHeight = window.innerHeight;
      if (canvas2.width != displayWidth || canvas2.height != displayHeight) {
        canvas2.width = displayWidth;
        canvas2.height = displayHeight;
        gl.viewport(0, 0, canvas2.width, canvas2.height);
      }
    }
    function render() {
      resizeCanvasToDisplaySize(canvas);
      gl.uniform2f(resolutionUniformLocation, canvas.width, canvas.height);
      gl.clearColor(0, 0, 0, 1);
      gl.clear(gl.COLOR_BUFFER_BIT);
      gl.uniform1f(timeUniformLocation, performance.now() / 1e3);
      gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4);
      requestAnimationFrame(render);
    }
    window.addEventListener("resize", () => {
      resizeCanvasToDisplaySize(canvas);
    });
    render();
  }

  // src/code/js/tesseract.ts
  window.onhashchange = function change() {
    const hash = location.hash.split("#").pop();
    document.body.className = "Active";
    document.body.className = [document.body.className, " ", hash].join("");
    if (hash.length) {
      note(900, { reverb: 1 / 8 });
    }
  };
  var logo_state = false;
  function setLogoState(newState) {
    logo_state = newState;
  }
  function getLogoState() {
    return logo_state;
  }
  var observer = new IntersectionObserver((entries, observer2) => {
    entries.forEach((entry) => {
      if (entry.isIntersecting) {
        document.getElementById("Back")?.classList.remove("Active");
      } else {
        document.getElementById("Back")?.classList.add("Active");
      }
    });
  });
  var Partners = document.getElementById("Partners");
  observer.observe(Partners);
  var Logo = document.getElementById("Logo")?.children[0];
  Logo?.addEventListener("click", async () => {
    observer.unobserve(Partners);
    await logoClick();
  });
  grid(document.getElementById("grid-dynamic"));
})();
//# sourceMappingURL=tesseract.js.map
