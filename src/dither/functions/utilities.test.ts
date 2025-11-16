import { describe, it, expect } from 'vitest';
import utilities, { randomInteger } from './utilities';

describe('utilities', () => {
  describe('randomInteger', () => {
    it('should generate integers within the specified range', () => {
      for (let i = 0; i < 100; i++) {
        const result = randomInteger(0, 10);
        expect(result).toBeGreaterThanOrEqual(0);
        expect(result).toBeLessThanOrEqual(10);
        expect(Number.isInteger(result)).toBe(true);
      }
    });

    it('should handle min and max being the same', () => {
      const result = randomInteger(5, 5);
      expect(result).toBe(5);
    });

    it('should handle range of 0-255 (typical color range)', () => {
      for (let i = 0; i < 50; i++) {
        const result = randomInteger(0, 255);
        expect(result).toBeGreaterThanOrEqual(0);
        expect(result).toBeLessThanOrEqual(255);
      }
    });

    it('should handle negative ranges', () => {
      for (let i = 0; i < 50; i++) {
        const result = randomInteger(-10, 10);
        expect(result).toBeGreaterThanOrEqual(-10);
        expect(result).toBeLessThanOrEqual(10);
      }
    });

    it('should handle large ranges', () => {
      for (let i = 0; i < 50; i++) {
        const result = randomInteger(0, 10000);
        expect(result).toBeGreaterThanOrEqual(0);
        expect(result).toBeLessThanOrEqual(10000);
      }
    });

    it('should be exported as part of default object', () => {
      expect(utilities.randomInteger).toBe(randomInteger);
    });

    it('should generate different values over multiple calls', () => {
      const results = new Set();
      // Generate 50 random numbers between 0-100
      // Very unlikely to get all the same number
      for (let i = 0; i < 50; i++) {
        results.add(randomInteger(0, 100));
      }
      // We expect at least some variation
      expect(results.size).toBeGreaterThan(1);
    });
  });
});
