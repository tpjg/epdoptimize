import { beforeAll } from 'vitest';

// Mock ImageData class
class MockImageData {
  public width: number;
  public height: number;
  public data: Uint8ClampedArray;

  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    this.data = new Uint8ClampedArray(width * height * 4);
  }
}

// Make ImageData available globally
if (typeof global !== 'undefined') {
  (global as any).ImageData = MockImageData;
}

// Mock Canvas 2D context for testing
class MockCanvasRenderingContext2D {
  private storedImageData: any = null;
  private canvasWidth: number = 0;
  private canvasHeight: number = 0;

  constructor(canvas: HTMLCanvasElement) {
    this.canvasWidth = canvas.width;
    this.canvasHeight = canvas.height;
  }

  getImageData(x: number, y: number, width: number, height: number): any {
    // If we have stored data, return a copy of it
    if (this.storedImageData &&
        this.storedImageData.width === width &&
        this.storedImageData.height === height) {
      // Return a deep copy so modifications don't affect the stored version
      const copy = new MockImageData(width, height) as any;
      copy.data.set(this.storedImageData.data);
      return copy;
    }

    // Otherwise create new empty imageData
    return new MockImageData(width, height) as any;
  }

  putImageData(imageData: any, dx: number, dy: number): void {
    // Store a deep copy of the imageData
    const stored = new MockImageData(imageData.width, imageData.height) as any;
    stored.data.set(imageData.data);
    this.storedImageData = stored;
  }

  clearRect(x: number, y: number, width: number, height: number): void {
    // Mock implementation
  }
}

beforeAll(() => {
  // Mock HTMLCanvasElement.prototype.getContext
  if (typeof HTMLCanvasElement !== 'undefined') {
    const contextCache = new WeakMap<HTMLCanvasElement, MockCanvasRenderingContext2D>();

    HTMLCanvasElement.prototype.getContext = function(
      contextId: string,
      options?: any
    ): any {
      if (contextId === '2d') {
        // Return cached context if it exists
        if (!contextCache.has(this)) {
          contextCache.set(this, new MockCanvasRenderingContext2D(this));
        }
        return contextCache.get(this);
      }
      return null;
    };
  }
});
