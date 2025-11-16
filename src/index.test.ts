import { describe, it, expect } from 'vitest';
import { getDefaultPalettes, getDeviceColors } from './index';

describe('index', () => {
  describe('getDefaultPalettes', () => {
    it('should return default palette for "default" input', () => {
      const palette = getDefaultPalettes('default');
      expect(palette).toEqual(['#000', '#fff']);
    });

    it('should return spectra6 palette', () => {
      const palette = getDefaultPalettes('spectra6');
      expect(palette).toEqual([
        '#191E21',
        '#e8e8e8',
        '#2157ba',
        '#125f20',
        '#b21318',
        '#efde44',
      ]);
    });

    it('should return acep palette', () => {
      const palette = getDefaultPalettes('acep');
      expect(palette).toEqual([
        '#191E21',
        '#F1F1F1',
        '#31318F',
        '#53A428',
        '#D20E13',
        '#B85E1C',
        '#F3CF11',
      ]);
    });

    it('should return gameboy palette', () => {
      const palette = getDefaultPalettes('gameboy');
      expect(palette).toEqual([
        '#0f380f',
        '#306230',
        '#8bac0f',
        '#9bbc0f',
      ]);
    });

    it('should be case insensitive', () => {
      expect(getDefaultPalettes('SPECTRA6')).toEqual(getDefaultPalettes('spectra6'));
      expect(getDefaultPalettes('Spectra6')).toEqual(getDefaultPalettes('spectra6'));
      expect(getDefaultPalettes('ACEP')).toEqual(getDefaultPalettes('acep'));
      expect(getDefaultPalettes('AcEp')).toEqual(getDefaultPalettes('acep'));
    });

    it('should return default palette for unknown palette name', () => {
      const palette = getDefaultPalettes('unknown-palette');
      expect(palette).toEqual(['#000', '#fff']);
    });

    it('should return default palette for empty string', () => {
      const palette = getDefaultPalettes('');
      expect(palette).toEqual(['#000', '#fff']);
    });

    it('should return arrays of hex color strings', () => {
      const palette = getDefaultPalettes('spectra6');
      expect(Array.isArray(palette)).toBe(true);
      palette.forEach(color => {
        expect(typeof color).toBe('string');
        expect(color).toMatch(/^#[0-9a-fA-F]+$/);
      });
    });

    it('should return different sized palettes for different displays', () => {
      const defaultPalette = getDefaultPalettes('default');
      const spectra6Palette = getDefaultPalettes('spectra6');
      const acepPalette = getDefaultPalettes('acep');

      expect(defaultPalette).toHaveLength(2);
      expect(spectra6Palette).toHaveLength(6);
      expect(acepPalette).toHaveLength(7);
    });
  });

  describe('getDeviceColors', () => {
    it('should return default device colors for "default" input', () => {
      const colors = getDeviceColors('default');
      expect(colors).toEqual(['#e6e6e6', '#212121']);
    });

    it('should return spectra6 device colors', () => {
      const colors = getDeviceColors('spectra6');
      expect(Array.isArray(colors)).toBe(true);
      expect(colors.length).toBeGreaterThan(0);
    });

    it('should return acep device colors', () => {
      const colors = getDeviceColors('acep');
      expect(Array.isArray(colors)).toBe(true);
      expect(colors.length).toBeGreaterThan(0);
    });

    it('should be case insensitive', () => {
      expect(getDeviceColors('SPECTRA6')).toEqual(getDeviceColors('spectra6'));
      expect(getDeviceColors('Spectra6')).toEqual(getDeviceColors('spectra6'));
    });

    it('should return default device colors for unknown name', () => {
      const colors = getDeviceColors('unknown-device');
      expect(colors).toEqual(['#e6e6e6', '#212121']);
    });

    it('should return arrays of hex color strings', () => {
      const colors = getDeviceColors('spectra6');
      expect(Array.isArray(colors)).toBe(true);
      colors.forEach(color => {
        expect(typeof color).toBe('string');
        expect(color).toMatch(/^#[0-9a-fA-F]+$/);
      });
    });
  });

  describe('palette and device colors relationship', () => {
    it('should have matching lengths for spectra6', () => {
      const palette = getDefaultPalettes('spectra6');
      const deviceColors = getDeviceColors('spectra6');
      expect(palette.length).toBe(deviceColors.length);
    });

    it('should have matching lengths for acep', () => {
      const palette = getDefaultPalettes('acep');
      const deviceColors = getDeviceColors('acep');
      expect(palette.length).toBe(deviceColors.length);
    });

    it('should have matching lengths for default', () => {
      const palette = getDefaultPalettes('default');
      const deviceColors = getDeviceColors('default');
      expect(palette.length).toBe(deviceColors.length);
    });
  });
});
