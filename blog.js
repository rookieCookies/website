window.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll("img[src^='wasm/']").forEach(img => {
    const src = img.getAttribute("src");

    // create a wrapper
    const wrapper = document.createElement("div");
    wrapper.className = "wasm-container";

    // create an iframe or canvas
    const iframe = document.createElement("iframe");
    iframe.src = src; // e.g. "wasm/draw-a-cube"
    iframe.width = img.getAttribute("width") || "640";
    iframe.height = img.getAttribute("height") || "480";
    iframe.loading = "lazy";
    iframe.style.border = "none";
    wrapper.appendChild(iframe);

    img.replaceWith(wrapper);
  });
});

document.addEventListener("DOMContentLoaded", () => {
  const videos = document.querySelectorAll("video[data-src]");
  const observer = new IntersectionObserver(entries => {
    for (const entry of entries) {
      if (entry.isIntersecting) {
        const vid = entry.target;
        vid.src = vid.dataset.src;
        observer.unobserve(vid);
      }
    }
  });

  videos.forEach(v => observer.observe(v));
});

/*
document.addEventListener("DOMContentLoaded", () => {
  document.querySelectorAll("img.blur-up").forEach(img => {
    const realSrcSet = img.dataset.srcset;
    const highRes = new Image();
    highRes.srcset = realSrcSet;
    highRes.sizes = img.sizes;

    highRes.onload = () => {
      img.srcset = realSrcSet;
      img.classList.add("loaded");
    };
  });
});
*/