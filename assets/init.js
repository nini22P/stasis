let isLoaded = false;

const handleClose = () => {
  if (isLoaded) {
    close();
  }
};

document.addEventListener('DOMContentLoaded', () => {
  loaded();
  setTimeout(() => {
    isLoaded = true;
  }, 500);
});

window.addEventListener('mousemove', handleClose);
window.addEventListener('keydown', handleClose);
window.addEventListener('click', handleClose);
window.addEventListener('touchstart', handleClose);
window.addEventListener('touchmove', handleClose);