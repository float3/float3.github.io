import("./index.js").catch((e) => console.error("Error importing `index.js`:", e))
import { transformLeftToRight, transformRightToLeft } from "./index.js"

/**
 * Copies the content of the input field with the given ID to the clipboard.
 * @param inputId - The ID of the input element to copy from.
 */
function copyToClipboard(inputId: string): void {
  const inputElement = document.getElementById(inputId) as HTMLInputElement | null;
  if (!inputElement) {
    alert(`Input element with ID "${inputId}" not found.`);
    return;
  }

  const textToCopy = inputElement.value;
  if (!navigator.clipboard) {
    const textarea = document.createElement('textarea');
    textarea.value = textToCopy;
    textarea.style.position = 'fixed';
    document.body.appendChild(textarea);
    textarea.focus();
    textarea.select();
    try {
      const successful = document.execCommand('copy');
      if (successful) {
        alert('Copied to clipboard!');
      } else {
        alert('Failed to copy.');
      }
    } catch (err) {
      alert('Error copying to clipboard.' + err);
    }
    document.body.removeChild(textarea);
    return;
  }

  navigator.clipboard.writeText(textToCopy).then(() => {
    alert('Copied to clipboard!');
  }).catch(() => {
    alert('Failed to copy.');
  });
}

declare global {
  interface Window {
    transformLeftToRight: typeof transformLeftToRight
    transformRightToLeft: typeof transformRightToLeft
    copyToClipboard: typeof copyToClipboard
  }
}

window.transformLeftToRight = transformLeftToRight
window.transformRightToLeft = transformRightToLeft
window.copyToClipboard = copyToClipboard;
