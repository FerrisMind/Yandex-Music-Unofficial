// Основной файл для приложения Яндекс.Музыки
console.log('Яндекс Музыка (Unofficial) загружается...');

// Отключение контекстного меню
document.addEventListener('contextmenu', function(event) {
  event.preventDefault();
});

// Можно добавить дополнительную логику здесь при необходимости
window.addEventListener("DOMContentLoaded", () => {
  console.log('Приложение загружено');
});
