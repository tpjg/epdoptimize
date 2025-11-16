import { describe, it, expect, beforeEach, vi } from 'vitest';
import { replaceColors as replaceColorsFunc } from './replaceColors';

// Helper to create a mock canvas with imageData
function createMockCanvas(width: number, height: number, fillColor?: number[]) {
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d')!;

  if (fillColor) {
    const imageData = ctx.getImageData(0, 0, width, height);
    for (let i = 0; i < imageData.data.length; i += 4) {
      imageData.data[i] = fillColor[0];     // R
      imageData.data[i + 1] = fillColor[1]; // G
      imageData.data[i + 2] = fillColor[2]; // B
      imageData.data[i + 3] = 255;          // A
    }
    ctx.putImageData(imageData, 0, 0);
  }

  return canvas;
}

describe('replaceColors', () => {
  let consoleWarnSpy: any;

  beforeEach(() => {
    consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});
  });

  it('should replace colors correctly', () => {
    const fromCanvas = createMockCanvas(2, 2, [255, 0, 0]); // Red
    const destCanvas = createMockCanvas(2, 2);

    const originalColors = ['#FF0000'];
    const replaceColors = ['#00FF00']; // Green

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });

    const destCtx = destCanvas.getContext('2d')!;
    const destImageData = destCtx.getImageData(0, 0, 2, 2);

    // All pixels should now be green
    for (let i = 0; i < destImageData.data.length; i += 4) {
      expect(destImageData.data[i]).toBe(0);     // R
      expect(destImageData.data[i + 1]).toBe(255); // G
      expect(destImageData.data[i + 2]).toBe(0);   // B
    }
  });

  it('should handle multiple color replacements', () => {
    const fromCanvas = createMockCanvas(4, 1);
    const ctx = fromCanvas.getContext('2d')!;
    const imageData = ctx.getImageData(0, 0, 4, 1);

    // Set different colors for each pixel
    // Pixel 0: Red
    imageData.data[0] = 255; imageData.data[1] = 0; imageData.data[2] = 0; imageData.data[3] = 255;
    // Pixel 1: Green
    imageData.data[4] = 0; imageData.data[5] = 255; imageData.data[6] = 0; imageData.data[7] = 255;
    // Pixel 2: Blue
    imageData.data[8] = 0; imageData.data[9] = 0; imageData.data[10] = 255; imageData.data[11] = 255;
    // Pixel 3: Red again
    imageData.data[12] = 255; imageData.data[13] = 0; imageData.data[14] = 0; imageData.data[15] = 255;

    ctx.putImageData(imageData, 0, 0);

    const destCanvas = createMockCanvas(4, 1);
    const originalColors = ['#FF0000', '#00FF00', '#0000FF'];
    const replaceColors = ['#000000', '#FFFFFF', '#808080']; // Black, White, Gray

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });

    const destCtx = destCanvas.getContext('2d')!;
    const destImageData = destCtx.getImageData(0, 0, 4, 1);

    // Pixel 0: should be black (red -> black)
    expect(destImageData.data[0]).toBe(0);
    expect(destImageData.data[1]).toBe(0);
    expect(destImageData.data[2]).toBe(0);

    // Pixel 1: should be white (green -> white)
    expect(destImageData.data[4]).toBe(255);
    expect(destImageData.data[5]).toBe(255);
    expect(destImageData.data[6]).toBe(255);

    // Pixel 2: should be gray (blue -> gray)
    expect(destImageData.data[8]).toBe(128);
    expect(destImageData.data[9]).toBe(128);
    expect(destImageData.data[10]).toBe(128);

    // Pixel 3: should be black (red -> black)
    expect(destImageData.data[12]).toBe(0);
    expect(destImageData.data[13]).toBe(0);
    expect(destImageData.data[14]).toBe(0);
  });

  it('should handle shorthand hex colors', () => {
    const fromCanvas = createMockCanvas(1, 1, [255, 0, 0]);
    const destCanvas = createMockCanvas(1, 1);

    const originalColors = ['#F00']; // Shorthand red
    const replaceColors = ['#0F0']; // Shorthand green

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });

    const destCtx = destCanvas.getContext('2d')!;
    const pixel = destCtx.getImageData(0, 0, 1, 1);

    expect(pixel.data[0]).toBe(0);
    expect(pixel.data[1]).toBe(255);
    expect(pixel.data[2]).toBe(0);
  });

  it('should preserve destination canvas dimensions', () => {
    const fromCanvas = createMockCanvas(10, 20, [0, 0, 0]);
    const destCanvas = createMockCanvas(5, 5); // Different initial size

    const originalColors = ['#000000'];
    const replaceColors = ['#FFFFFF'];

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });

    expect(destCanvas.width).toBe(10);
    expect(destCanvas.height).toBe(20);
  });

  it('should warn when colors do not match exactly', () => {
    const fromCanvas = createMockCanvas(2, 2, [255, 0, 0]); // Red
    const destCanvas = createMockCanvas(2, 2);

    const originalColors = ['#00FF00']; // Green (not in image)
    const replaceColors = ['#0000FF']; // Blue

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });

    // Should have warned about unmatched pixels
    expect(consoleWarnSpy).toHaveBeenCalled();
    expect(consoleWarnSpy.mock.calls[0][0]).toContain('pixels were not replaced');
  });

  it('should not warn when all colors match', () => {
    const fromCanvas = createMockCanvas(2, 2, [255, 0, 0]);
    const destCanvas = createMockCanvas(2, 2);

    const originalColors = ['#FF0000'];
    const replaceColors = ['#00FF00'];

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });

    expect(consoleWarnSpy).not.toHaveBeenCalled();
  });

  it('should handle empty canvas', () => {
    const fromCanvas = createMockCanvas(0, 0);
    const destCanvas = createMockCanvas(0, 0);

    const originalColors = ['#FF0000'];
    const replaceColors = ['#00FF00'];

    // Should not throw
    expect(() => {
      replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });
    }).not.toThrow();
  });

  it('should handle 1x1 canvas', () => {
    const fromCanvas = createMockCanvas(1, 1, [25, 30, 33]);
    const destCanvas = createMockCanvas(1, 1);

    const originalColors = ['#191E21'];
    const replaceColors = ['#FFFFFF'];

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });

    const destCtx = destCanvas.getContext('2d')!;
    const pixel = destCtx.getImageData(0, 0, 1, 1);

    expect(pixel.data[0]).toBe(255);
    expect(pixel.data[1]).toBe(255);
    expect(pixel.data[2]).toBe(255);
  });

  it('should work with spectra6 palette colors', () => {
    const fromCanvas = createMockCanvas(3, 2);
    const ctx = fromCanvas.getContext('2d')!;
    const imageData = ctx.getImageData(0, 0, 3, 2);

    // Fill with spectra6 calibrated colors
    const spectra6Colors = [
      [25, 30, 33],     // black
      [232, 232, 232],  // white
      [33, 87, 186],    // blue
    ];

    for (let i = 0; i < 6; i++) {
      const colorIndex = i % 3;
      const pixelIndex = i * 4;
      imageData.data[pixelIndex] = spectra6Colors[colorIndex][0];
      imageData.data[pixelIndex + 1] = spectra6Colors[colorIndex][1];
      imageData.data[pixelIndex + 2] = spectra6Colors[colorIndex][2];
      imageData.data[pixelIndex + 3] = 255;
    }

    ctx.putImageData(imageData, 0, 0);

    const destCanvas = createMockCanvas(3, 2);

    const originalColors = ['#191E21', '#e8e8e8', '#2157ba'];
    const deviceColors = ['#000000', '#FFFFFF', '#0000FF'];

    replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors: deviceColors });

    const destCtx = destCanvas.getContext('2d')!;
    const destData = destCtx.getImageData(0, 0, 3, 2);

    // First pixel should be pure black
    expect(destData.data[0]).toBe(0);
    expect(destData.data[1]).toBe(0);
    expect(destData.data[2]).toBe(0);

    // Second pixel should be pure white
    expect(destData.data[4]).toBe(255);
    expect(destData.data[5]).toBe(255);
    expect(destData.data[6]).toBe(255);

    // Third pixel should be pure blue
    expect(destData.data[8]).toBe(0);
    expect(destData.data[9]).toBe(0);
    expect(destData.data[10]).toBe(255);
  });

  it('should handle mismatched palette lengths gracefully', () => {
    const fromCanvas = createMockCanvas(1, 1, [255, 0, 0]);
    const destCanvas = createMockCanvas(1, 1);

    const originalColors = ['#FF0000', '#00FF00'];
    const replaceColors = ['#000000']; // Only one replacement color

    // Should not throw, but may not replace all colors
    expect(() => {
      replaceColorsFunc(fromCanvas, destCanvas, { originalColors, replaceColors });
    }).not.toThrow();
  });
});
