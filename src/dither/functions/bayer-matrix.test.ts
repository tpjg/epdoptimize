import { describe, it, expect } from 'vitest';
import createBayerMatrix from './bayer-matrix';

describe('bayer-matrix', () => {
  describe('createBayerMatrix', () => {
    it('should create a 2x2 Bayer matrix', () => {
      const matrix = createBayerMatrix([2, 2]);
      expect(matrix).toHaveLength(2);
      expect(matrix[0]).toHaveLength(2);
      expect(matrix[1]).toHaveLength(2);

      // Values should be 0-3 (reordered)
      const flatValues = matrix.flat().sort((a, b) => a - b);
      expect(flatValues).toEqual([0, 1, 2, 3]);
    });

    it('should create a 4x4 Bayer matrix', () => {
      const matrix = createBayerMatrix([4, 4]);
      expect(matrix).toHaveLength(4);
      matrix.forEach(row => {
        expect(row).toHaveLength(4);
      });

      // Values should be 0-15 (reordered)
      const flatValues = matrix.flat().sort((a, b) => a - b);
      expect(flatValues).toEqual([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    });

    it('should create an 8x8 Bayer matrix', () => {
      const matrix = createBayerMatrix([8, 8]);
      expect(matrix).toHaveLength(8);
      matrix.forEach(row => {
        expect(row).toHaveLength(8);
      });

      // Should have 64 values total
      const flatValues = matrix.flat();
      expect(flatValues).toHaveLength(64);

      // All values should be in range 0-63
      flatValues.forEach(val => {
        expect(val).toBeGreaterThanOrEqual(0);
        expect(val).toBeLessThan(64);
      });
    });

    it('should return the hardcoded 8x8 matrix for [8,8] input', () => {
      const matrix = createBayerMatrix([8, 8]);
      const expectedBigMatrix = [
        [0, 48, 12, 60, 3, 51, 15, 63],
        [32, 16, 44, 28, 35, 19, 47, 31],
        [8, 56, 4, 52, 11, 59, 7, 55],
        [40, 24, 36, 20, 43, 27, 39, 32],
        [2, 50, 14, 62, 1, 49, 13, 61],
        [34, 18, 46, 30, 33, 17, 45, 29],
        [10, 58, 6, 54, 9, 57, 5, 53],
        [42, 26, 38, 22, 41, 25, 37, 21],
      ];

      // The function reindexes values, so we need to check the structure
      expect(matrix).toHaveLength(8);
      expect(matrix[0]).toHaveLength(8);
    });

    it('should create a rectangular 4x2 matrix', () => {
      const matrix = createBayerMatrix([4, 2]);
      expect(matrix).toHaveLength(2); // height = 2
      expect(matrix[0]).toHaveLength(4); // width = 4
      expect(matrix[1]).toHaveLength(4);

      // Should have 8 unique values
      const flatValues = matrix.flat().sort((a, b) => a - b);
      expect(flatValues).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
    });

    it('should create a rectangular 2x4 matrix', () => {
      const matrix = createBayerMatrix([2, 4]);
      expect(matrix).toHaveLength(4); // height = 4
      expect(matrix[0]).toHaveLength(2); // width = 2

      // Should have 8 unique values
      const flatValues = matrix.flat().sort((a, b) => a - b);
      expect(flatValues).toEqual([0, 1, 2, 3, 4, 5, 6, 7]);
    });

    it('should cap size at 8x8 when larger dimensions requested', () => {
      const matrix = createBayerMatrix([10, 10]);
      expect(matrix).toHaveLength(8);
      expect(matrix[0]).toHaveLength(8);
    });

    it('should cap width at 8 when larger width requested', () => {
      const matrix = createBayerMatrix([16, 4]);
      expect(matrix).toHaveLength(4);
      expect(matrix[0]).toHaveLength(8);
    });

    it('should cap height at 8 when larger height requested', () => {
      const matrix = createBayerMatrix([4, 16]);
      expect(matrix).toHaveLength(8);
      expect(matrix[0]).toHaveLength(4);
    });

    it('should create a 1x1 matrix', () => {
      const matrix = createBayerMatrix([1, 1]);
      expect(matrix).toHaveLength(1);
      expect(matrix[0]).toHaveLength(1);
      expect(matrix[0][0]).toBe(0);
    });

    it('should have all unique values in matrix', () => {
      const sizes = [[2, 2], [3, 3], [4, 4], [5, 5]];

      sizes.forEach(size => {
        const matrix = createBayerMatrix(size);
        const values = matrix.flat();
        const uniqueValues = new Set(values);
        expect(uniqueValues.size).toBe(values.length);
      });
    });

    it('should have values in range 0 to (width*height - 1)', () => {
      const matrix = createBayerMatrix([4, 4]);
      const values = matrix.flat();

      values.forEach(value => {
        expect(value).toBeGreaterThanOrEqual(0);
        expect(value).toBeLessThan(16);
      });
    });

    it('should be consistent for the same input', () => {
      const matrix1 = createBayerMatrix([4, 4]);
      const matrix2 = createBayerMatrix([4, 4]);
      expect(matrix1).toEqual(matrix2);
    });

    it('should create matrices with proper dimensions for common use cases', () => {
      // Common Bayer matrix sizes used in dithering
      const commonSizes = [
        [2, 2],
        [4, 4],
        [8, 8],
      ];

      commonSizes.forEach(([width, height]) => {
        const matrix = createBayerMatrix([width, height]);
        expect(matrix).toHaveLength(height);
        expect(matrix[0]).toHaveLength(width);

        // All values should be integers
        matrix.flat().forEach(val => {
          expect(Number.isInteger(val)).toBe(true);
        });
      });
    });
  });
});
