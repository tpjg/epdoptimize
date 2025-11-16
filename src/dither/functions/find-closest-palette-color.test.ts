import { describe, it, expect } from 'vitest';
import findClosestPaletteColor from './find-closest-palette-color';

describe('find-closest-palette-color', () => {
  describe('findClosestPaletteColor', () => {
    it('should find exact color match in palette', () => {
      const palette = [
        [255, 0, 0],    // red
        [0, 255, 0],    // green
        [0, 0, 255],    // blue
      ];
      const pixel = [255, 0, 0, 255];
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([255, 0, 0, 255]);
    });

    it('should find closest color when no exact match', () => {
      const palette = [
        [0, 0, 0],      // black
        [255, 255, 255], // white
      ];
      const pixel = [100, 100, 100, 255]; // gray, closer to black
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([0, 0, 0, 255]);
    });

    it('should find closest color for light gray (closer to white)', () => {
      const palette = [
        [0, 0, 0],      // black
        [255, 255, 255], // white
      ];
      const pixel = [200, 200, 200, 255]; // light gray, closer to white
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([255, 255, 255, 255]);
    });

    it('should handle single color palette', () => {
      const palette = [[128, 128, 128]];
      const pixel = [255, 0, 0, 255];
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([128, 128, 128, 255]);
    });

    it('should add alpha channel if not present in palette color', () => {
      const palette = [
        [255, 0, 0],    // red without alpha
      ];
      const pixel = [255, 0, 0, 255];
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([255, 0, 0, 255]);
      expect(result.length).toBe(4);
    });

    it('should preserve alpha channel if present in palette color', () => {
      const palette = [
        [255, 0, 0, 128], // red with 50% alpha
      ];
      const pixel = [255, 0, 0, 255];
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([255, 0, 0, 128]);
    });

    it('should work with spectra6 palette colors', () => {
      const palette = [
        [25, 30, 33],     // black
        [232, 232, 232],  // white
        [33, 87, 186],    // blue
        [18, 95, 32],     // green
        [178, 19, 24],    // red
        [239, 222, 68],   // yellow
      ];

      // Test a color close to red
      const redishPixel = [200, 20, 20, 255];
      const result = findClosestPaletteColor(redishPixel, palette);
      expect(result).toEqual([178, 19, 24, 255]);
    });

    it('should handle edge case of identical palette colors', () => {
      const palette = [
        [100, 100, 100],
        [100, 100, 100],
        [200, 200, 200],
      ];
      const pixel = [110, 110, 110, 255];
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([100, 100, 100, 255]);
    });

    it('should calculate Euclidean distance correctly', () => {
      const palette = [
        [0, 0, 0],       // distance to [128,128,128] = sqrt(3*128^2) ≈ 221.7
        [128, 0, 0],     // distance to [128,128,128] = sqrt(128^2+128^2) ≈ 181.0
        [128, 128, 0],   // distance to [128,128,128] = sqrt(128^2) = 128
        [128, 128, 128], // distance to [128,128,128] = 0 (exact match)
      ];
      const pixel = [128, 128, 128, 255];
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([128, 128, 128, 255]);
    });

    it('should handle pixels at color boundaries', () => {
      const palette = [
        [0, 0, 0],
        [255, 255, 255],
      ];

      // Test pure black
      let result = findClosestPaletteColor([0, 0, 0, 255], palette);
      expect(result).toEqual([0, 0, 0, 255]);

      // Test pure white
      result = findClosestPaletteColor([255, 255, 255, 255], palette);
      expect(result).toEqual([255, 255, 255, 255]);
    });

    it('should work with gameboy palette', () => {
      const palette = [
        [15, 56, 15],
        [48, 98, 48],
        [139, 172, 15],
        [155, 188, 15],
      ];

      const darkPixel = [20, 60, 20, 255];
      const result = findClosestPaletteColor(darkPixel, palette);
      expect(result).toEqual([15, 56, 15, 255]);
    });

    it('should handle colors with different RGB components', () => {
      const palette = [
        [255, 0, 0],   // pure red
        [0, 255, 0],   // pure green
        [0, 0, 255],   // pure blue
      ];

      // Greenish pixel
      const pixel = [50, 200, 50, 255];
      const result = findClosestPaletteColor(pixel, palette);
      expect(result).toEqual([0, 255, 0, 255]);
    });
  });
});
