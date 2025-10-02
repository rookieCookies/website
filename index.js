const scrollArrow = document.getElementById('scrollArrow');
let idleTimer;

// Show arrow after 15s of no scroll
function startIdleTimer() {
    clearTimeout(idleTimer);
    idleTimer = setTimeout(() => {
        updateArrowVisibility()
    }, 5000); // 15000ms = 15 seconds
}


function updateArrowVisibility() {
    if (document.documentElement.scrollHeight > window.innerHeight &&
        window.scrollY + window.innerHeight < document.documentElement.scrollHeight - 1) {
        scrollArrow.classList.add('visible');
    } else {
        scrollArrow.classList.remove('visible');
    }
}


// Hide arrow immediately on scroll
function onScroll() {
    scrollArrow.classList.remove('visible');
    startIdleTimer();
}

// Scroll one full viewport down when clicked
scrollArrow.addEventListener('click', () => {
    window.scrollBy({
        top: window.innerHeight,
        left: 0,
        behavior: 'smooth'
    });
});


// Initial check
onScroll();

// Update on scroll & resize
window.addEventListener('scroll', onScroll);
window.addEventListener('resize', updateArrowVisibility);