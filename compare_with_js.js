#!/usr/bin/env node

/**
 * Compare JS implementation with a test image
 * Usage: node compare_with_js.js <input> <output> <algorithm> <palette>
 */

const fs = require('fs');
const path = require('path');
const { createCanvas, loadImage } = require('canvas');

// Import the built library
const { ditherImage, getDefaultPalettes, getDeviceColors, replaceColors } = require('./dist/index.cjs.js');

async function main() {
    const args = process.argv.slice(2);

    if (args.length < 2) {
        console.error('Usage: node compare_with_js.js <input> <output> [algorithm] [palette]');
        process.exit(1);
    }

    const inputPath = args[0];
    const outputPath = args[1];
    const algorithm = args[2] || 'floydSteinberg';
    const paletteName = args[3] || 'spectra6';

    console.log(`Input: ${inputPath}`);
    console.log(`Output: ${outputPath}`);
    console.log(`Algorithm: ${algorithm}`);
    console.log(`Palette: ${paletteName}`);

    // Load image
    const img = await loadImage(inputPath);

    // Create canvases
    const sourceCanvas = createCanvas(img.width, img.height);
    const sourceCtx = sourceCanvas.getContext('2d');
    sourceCtx.drawImage(img, 0, 0);

    const outputCanvas = createCanvas(img.width, img.height);
    const deviceCanvas = createCanvas(img.width, img.height);

    // Get palette
    const palette = getDefaultPalettes(paletteName);
    const deviceColors = getDeviceColors(paletteName);

    console.log(`Palette has ${palette.length} colors`);

    // Map algorithm name from Rust CLI to JS
    const algorithmMap = {
        'floyd-steinberg': 'floydSteinberg',
        'false-floyd-steinberg': 'falseFloydSteinberg',
        'jarvis': 'jarvis',
        'stucki': 'stucki',
        'burkes': 'burkes',
        'sierra3': 'sierra3',
        'sierra2': 'sierra2',
        'sierra24a': 'Sierra2-4A',
        'ordered': 'bayer',
        'random-rgb': 'randomRgb',
        'random-bw': 'randomBw',
        'none': 'quantizationOnly'
    };

    const jsAlgorithm = algorithmMap[algorithm] || algorithm;

    // Prepare options
    const options = {
        ditheringType: 'errorDiffusion',
        errorDiffusionMatrix: jsAlgorithm,
        palette: palette,
        serpentine: false
    };

    if (algorithm === 'ordered') {
        options.ditheringType = 'ordered';
        options.orderedDitheringMatrix = [4, 4];
    } else if (algorithm === 'random-rgb' || algorithm === 'random-bw') {
        options.ditheringType = 'random';
        options.randomDitheringType = algorithm === 'random-bw' ? 'blackAndWhite' : 'rgb';
    } else if (algorithm === 'none') {
        options.ditheringType = 'quantizationOnly';
    }

    console.log('Dithering with JS...');
    const startTime = Date.now();

    // Dither
    await ditherImage(sourceCanvas, outputCanvas, options);

    // Replace colors
    replaceColors(outputCanvas, deviceCanvas, {
        originalColors: palette,
        replaceColors: deviceColors
    });

    const endTime = Date.now();
    console.log(`JS processing time: ${endTime - startTime}ms`);

    // Save output
    const out = fs.createWriteStream(outputPath);
    const stream = deviceCanvas.createPNGStream();
    stream.pipe(out);

    await new Promise((resolve, reject) => {
        out.on('finish', resolve);
        out.on('error', reject);
    });

    console.log(`Saved to ${outputPath}`);
}

main().catch(err => {
    console.error('Error:', err);
    process.exit(1);
});
