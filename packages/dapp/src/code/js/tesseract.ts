import { logoClick } from "./logo-click";
import { note } from "./note";

window.onhashchange = function change() {
  const hash = location.hash.split("#").pop();

  document.body.className = "Active";
  document.body.className = [document.body.className, " ", hash].join("");
  if (hash.length) {
    note(900, { reverb: 1 / 8 });
  }
};

export let logo_state = false;

export function setLogoState(newState: boolean) {
  logo_state = newState;
}
export function getLogoState() {
  return logo_state;
}

// Create a new IntersectionObserver
const observer = new IntersectionObserver((entries, observer) => {
  entries.forEach((entry) => {
    // If the entry (section#Partners) is in the viewport
    if (entry.isIntersecting) {
      // Hide the background element
      document.getElementById("Back")?.classList.remove("Active");
      // Stop observing the entry
    } else {
      // Show the background element
      document.getElementById("Back")?.classList.add("Active");
    }
  });
});

// Start observing an element (section#Partners)
const Partners = document.getElementById("Partners");
observer.observe(Partners);

const Logo = document.getElementById("Logo")?.children[0];
Logo?.addEventListener("click", async () => {
  observer.unobserve(Partners);
  await logoClick();
});

import { grid } from "./the-grid";
grid(document.getElementById("grid-dynamic"));