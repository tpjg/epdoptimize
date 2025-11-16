import { describe, it, expect } from 'vitest';
import colorHelpers, { hexToRgb } from './color-helpers';

describe('color-helpers', () => {
  describe('hexToRgb', () => {
    it('should convert full hex to RGB', () => {
      expect(hexToRgb('#FF0000')).toEqual([255, 0, 0]);
      expect(hexToRgb('#00FF00')).toEqual([0, 255, 0]);
      expect(hexToRgb('#0000FF')).toEqual([0, 0, 255]);
      expect(hexToRgb('#FFFFFF')).toEqual([255, 255, 255]);
      expect(hexToRgb('#000000')).toEqual([0, 0, 0]);
    });

    it('should convert shorthand hex to RGB', () => {
      expect(hexToRgb('#F00')).toEqual([255, 0, 0]);
      expect(hexToRgb('#0F0')).toEqual([0, 255, 0]);
      expect(hexToRgb('#00F')).toEqual([0, 0, 255]);
      expect(hexToRgb('#FFF')).toEqual([255, 255, 255]);
      expect(hexToRgb('#000')).toEqual([0, 0, 0]);
    });

    it('should handle hex without # prefix', () => {
      expect(hexToRgb('FF0000')).toEqual([255, 0, 0]);
      expect(hexToRgb('F00')).toEqual([255, 0, 0]);
    });

    it('should handle lowercase hex', () => {
      expect(hexToRgb('#ff0000')).toEqual([255, 0, 0]);
      expect(hexToRgb('#f00')).toEqual([255, 0, 0]);
      expect(hexToRgb('ff0000')).toEqual([255, 0, 0]);
    });

    it('should handle mixed case hex', () => {
      expect(hexToRgb('#Ff0000')).toEqual([255, 0, 0]);
      expect(hexToRgb('#aAbBcC')).toEqual([170, 187, 204]);
    });

    it('should handle specific color values', () => {
      expect(hexToRgb('#191E21')).toEqual([25, 30, 33]);
      expect(hexToRgb('#e8e8e8')).toEqual([232, 232, 232]);
      expect(hexToRgb('#2157ba')).toEqual([33, 87, 186]);
    });

    it('should return null for invalid hex strings', () => {
      expect(hexToRgb('GGGGGG')).toBeNull();
      expect(hexToRgb('not-a-color')).toBeNull();
      expect(hexToRgb('12345')).toBeNull(); // Wrong length
    });

    it('should be exported as part of default object', () => {
      expect(colorHelpers.hexToRgb).toBe(hexToRgb);
    });
  });
});
