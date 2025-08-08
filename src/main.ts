// Основной файл для приложения Яндекс Музыки
console.log('Яндекс Музыка (Unofficial) загружается...');

// Полное отключение контекстного меню
function disableContextMenu() {
  // Отключение на уровне документа
  document.addEventListener('contextmenu', function(event) {
    event.preventDefault();
    event.stopPropagation();
    return false;
  }, true);
  
  // Отключение на уровне window
  window.addEventListener('contextmenu', function(event) {
    event.preventDefault();
    event.stopPropagation();
    return false;
  }, true);
  
  // Отключение для всех элементов
  document.addEventListener('DOMContentLoaded', function() {
    const allElements = document.querySelectorAll('*');
    allElements.forEach(element => {
      element.addEventListener('contextmenu', function(event) {
        event.preventDefault();
        event.stopPropagation();
        return false;
      }, true);
    });
  });
  
  // Дополнительная защита через CSS
  const style = document.createElement('style');
  style.textContent = `
    * {
      -webkit-user-select: none;
      -moz-user-select: none;
      -ms-user-select: none;
      user-select: none;
    }
  `;
  document.head.appendChild(style);
}

// Применяем отключение контекстного меню
disableContextMenu();

// Можно добавить дополнительную логику здесь при необходимости
window.addEventListener("DOMContentLoaded", () => {
  console.log('Приложение загружено');
  // Повторное применение отключения после загрузки DOM
  disableContextMenu();
});
