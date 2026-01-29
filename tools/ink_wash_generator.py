"""
Procedural Ink-Wash Landscape Generator
Generates images in the style of ancient Chinese scroll paintings and maps.

Inspired by traditional Shan Shui (山水) painting techniques:
- Layered noise for mountain terrain with atmospheric perspective
- Ink contour strokes (cun fa / 皴法) from height gradients
- Sparse tree/pine silhouettes
- Misty watercolor bleed effects
- Aged paper texture with vignette

Usage:
    python ink_wash_generator.py -o output.png -W 1024 -H 768 -s 42
    python ink_wash_generator.py -o batch.png -n 5  # Generate 5 images

Requirements:
    pip install pillow numpy
"""

import numpy as np
from PIL import Image, ImageDraw, ImageFilter
import random
import math
import argparse
from pathlib import Path
from typing import Dict, List, Optional, Tuple


def lerp(a: float, b: float, t: float) -> float:
    """Linear interpolation between a and b."""
    return a + (b - a) * t


def smoothstep(t: float) -> float:
    """Smooth interpolation curve."""
    return t * t * (3 - 2 * t)


def fade(t: float) -> float:
    """Perlin noise fade function."""
    return t * t * t * (t * (t * 6 - 15) + 10)


def clamp(value: float, min_value: float, max_value: float) -> float:
    return max(min_value, min(max_value, value))


class PerlinNoise:
    """2D Perlin noise generator."""

    def __init__(self, seed: int = None):
        if seed is not None:
            random.seed(seed)
            np.random.seed(seed)

        # Generate permutation table
        self.perm = list(range(256))
        random.shuffle(self.perm)
        self.perm = self.perm + self.perm  # Double it for overflow

        # Gradient vectors
        self.gradients = [(math.cos(2 * math.pi * i / 8), math.sin(2 * math.pi * i / 8))
                          for i in range(8)]

    def _gradient(self, hash_val: int, x: float, y: float) -> float:
        g = self.gradients[hash_val % 8]
        return g[0] * x + g[1] * y

    def noise(self, x: float, y: float) -> float:
        """Generate noise value at (x, y)."""
        # Grid cell coordinates
        xi = int(x) & 255
        yi = int(y) & 255

        # Relative position in cell
        xf = x - int(x)
        yf = y - int(y)

        # Fade curves
        u = fade(xf)
        v = fade(yf)

        # Hash coordinates of corners
        aa = self.perm[self.perm[xi] + yi]
        ab = self.perm[self.perm[xi] + yi + 1]
        ba = self.perm[self.perm[xi + 1] + yi]
        bb = self.perm[self.perm[xi + 1] + yi + 1]

        # Gradient dot products
        x1 = lerp(self._gradient(aa, xf, yf), self._gradient(ba, xf - 1, yf), u)
        x2 = lerp(self._gradient(ab, xf, yf - 1), self._gradient(bb, xf - 1, yf - 1), u)

        return lerp(x1, x2, v)

    def octave_noise(self, x: float, y: float, octaves: int = 4,
                     persistence: float = 0.5, lacunarity: float = 2.0) -> float:
        """Generate multi-octave noise (fractal Brownian motion)."""
        total = 0.0
        frequency = 1.0
        amplitude = 1.0
        max_value = 0.0

        for _ in range(octaves):
            total += self.noise(x * frequency, y * frequency) * amplitude
            max_value += amplitude
            amplitude *= persistence
            frequency *= lacunarity

        return total / max_value


class InkWashGenerator:
    """Generates ink-wash style landscape images."""

    # Color palettes inspired by traditional Chinese paintings
    PAPER_COLORS = [
        (245, 235, 220),  # Warm cream
        (240, 230, 210),  # Aged parchment
        (235, 225, 200),  # Old scroll
        (250, 240, 225),  # Light rice paper
    ]

    INK_COLORS = [
        (30, 25, 20),     # Deep black ink
        (45, 40, 35),     # Warm black
        (35, 30, 25),     # Neutral ink
    ]

    WASH_TINTS = [
        (80, 90, 70),     # Moss green
        (100, 85, 70),    # Warm brown
        (70, 80, 90),     # Cool blue-grey
        (90, 80, 65),     # Ochre
    ]

    ELEMENT_PALETTES: Dict[str, List[Tuple[int, int, int]]] = {
        "metal": [(210, 210, 220), (190, 200, 215), (160, 170, 185)],
        "wood": [(90, 130, 80), (70, 110, 70), (110, 150, 95)],
        "water": [(85, 110, 140), (70, 90, 120), (60, 80, 105)],
        "fire": [(150, 80, 60), (180, 90, 60), (130, 60, 50)],
        "earth": [(120, 100, 70), (140, 115, 80), (100, 85, 60)],
        "default": [],
    }

    TIME_OF_DAY_PRESETS: Dict[str, Dict[str, float]] = {
        "dawn": {"brightness": 1.05, "contrast": 1.05, "warmth": 0.12, "vignette": 1.1},
        "day": {"brightness": 1.08, "contrast": 1.0, "warmth": 0.02, "vignette": 1.0},
        "dusk": {"brightness": 0.98, "contrast": 1.08, "warmth": 0.18, "vignette": 1.2},
        "night": {"brightness": 0.85, "contrast": 1.1, "warmth": -0.08, "vignette": 1.3},
        "none": {"brightness": 1.0, "contrast": 1.0, "warmth": 0.0, "vignette": 1.0},
    }

    TERRAIN_TYPES = ["mountains", "river_valley", "lakeside", "cliffs", "plains", "cascades", "world_map"]

    def __init__(
        self,
        width: int = 1024,
        height: int = 768,
        seed: int = None,
        element_palette: Optional[str] = None,
        time_of_day: str = "none",
        weather: str = "none",
        structure_density: float = 1.0,
        celestial_glow: float = 0.0,
        show_seal: bool = True,
        terrain: str = "mountains",
    ):
        self.width = width
        self.height = height
        self.seed = seed if seed is not None else random.randint(0, 999999)
        self.element_palette = (element_palette or "default").lower()
        self.time_of_day = (time_of_day or "none").lower()
        self.weather = (weather or "none").lower()
        self.structure_density = clamp(structure_density, 0.0, 2.0)
        self.celestial_glow = clamp(celestial_glow, 0.0, 1.0)
        self.show_seal = show_seal
        self.terrain = (terrain or "mountains").lower()

        random.seed(self.seed)
        np.random.seed(self.seed)

        self.noise = PerlinNoise(self.seed)
        self.paper_color = random.choice(self.PAPER_COLORS)
        self.ink_color = random.choice(self.INK_COLORS)
        self.wash_tint = random.choice(self.WASH_TINTS)
        self.palette_override = self.ELEMENT_PALETTES.get(self.element_palette, [])

    def generate_height_map(self) -> np.ndarray:
        """Generate terrain height map. Shape varies by self.terrain type."""
        if self.terrain == "river_valley":
            return self._height_map_river_valley()
        elif self.terrain == "lakeside":
            return self._height_map_lakeside()
        elif self.terrain == "cliffs":
            return self._height_map_cliffs()
        elif self.terrain == "plains":
            return self._height_map_plains()
        elif self.terrain == "cascades":
            return self._height_map_cascades()
        elif self.terrain == "world_map":
            return self._height_map_world_map()
        else:
            return self._height_map_mountains()

    def _height_map_mountains(self) -> np.ndarray:
        """Classic layered mountain height map."""
        height_map = np.zeros((self.height, self.width), dtype=np.float32)

        layers = [
            {"scale": 0.003, "octaves": 5, "weight": 0.8, "y_offset": 0.05, "peak_boost": 1.5},
            {"scale": 0.005, "octaves": 4, "weight": 0.9, "y_offset": 0.2, "peak_boost": 1.3},
            {"scale": 0.007, "octaves": 4, "weight": 1.0, "y_offset": 0.4, "peak_boost": 1.2},
            {"scale": 0.01, "octaves": 3, "weight": 0.6, "y_offset": 0.6, "peak_boost": 1.0},
        ]

        for layer in layers:
            layer_noise = PerlinNoise(self.seed + hash(str(layer["scale"])) % 10000)

            for y in range(self.height):
                y_norm = y / self.height
                horizon_factor = max(0.0, 1.0 - (y_norm - layer["y_offset"]) * 1.8)
                horizon_factor = smoothstep(horizon_factor)
                horizon_factor = math.pow(max(0.0, horizon_factor), 0.7)

                for x in range(self.width):
                    noise_val = layer_noise.octave_noise(
                        x * layer["scale"],
                        y * layer["scale"] * 0.6,
                        octaves=layer["octaves"],
                        persistence=0.45
                    )
                    noise_val = max(0.0, min(1.0, (noise_val + 1.0) / 2.0))
                    noise_val = math.pow(noise_val, 1.0 / layer["peak_boost"])
                    height_map[y, x] += noise_val * horizon_factor * layer["weight"]

        height_map = (height_map - height_map.min()) / (height_map.max() - height_map.min() + 0.001)
        return height_map

    def _height_map_river_valley(self) -> np.ndarray:
        """Mountains on sides with a river channel winding through the center."""
        height_map = np.zeros((self.height, self.width), dtype=np.float32)
        terrain_noise = PerlinNoise(self.seed)
        river_noise = PerlinNoise(self.seed + 777)

        for y in range(self.height):
            y_norm = y / self.height
            # River center meanders using noise
            river_center = 0.5 + river_noise.octave_noise(y * 0.005, 0.0, octaves=3) * 0.2
            for x in range(self.width):
                x_norm = x / self.width
                # Distance from river center
                dist = abs(x_norm - river_center)
                # Valley shape: low in center, high on sides
                valley = smoothstep(clamp(dist * 3.0, 0.0, 1.0))
                # Mountain noise on the sides
                noise_val = terrain_noise.octave_noise(x * 0.005, y * 0.004, octaves=4, persistence=0.45)
                noise_val = max(0.0, (noise_val + 1.0) / 2.0)
                # Horizon fade for upper portion
                horizon = smoothstep(clamp(1.0 - y_norm * 1.2, 0.0, 1.0))
                height_map[y, x] = valley * noise_val * (0.4 + horizon * 0.6)

        height_map = (height_map - height_map.min()) / (height_map.max() - height_map.min() + 0.001)
        # Store river path for later rendering
        self._river_center_func = lambda yy: 0.5 + river_noise.octave_noise(yy * 0.005, 0.0, octaves=3) * 0.2
        return height_map

    def _height_map_lakeside(self) -> np.ndarray:
        """Mountains in the background with a flat lake in the lower portion."""
        height_map = np.zeros((self.height, self.width), dtype=np.float32)
        terrain_noise = PerlinNoise(self.seed)

        for y in range(self.height):
            y_norm = y / self.height
            for x in range(self.width):
                noise_val = terrain_noise.octave_noise(x * 0.004, y * 0.003, octaves=5, persistence=0.5)
                noise_val = max(0.0, (noise_val + 1.0) / 2.0)
                # Mountains only in upper third, lake below
                mountain_mask = smoothstep(clamp(1.0 - y_norm * 1.8, 0.0, 1.0))
                # Shore line with gentle slope
                shore = smoothstep(clamp((0.55 - y_norm) * 4.0, 0.0, 1.0))
                height_map[y, x] = noise_val * mountain_mask * 0.8 + shore * 0.2

        height_map = (height_map - height_map.min()) / (height_map.max() - height_map.min() + 0.001)
        return height_map

    def _height_map_cliffs(self) -> np.ndarray:
        """Dramatic steep cliff faces with sharp vertical drops."""
        height_map = np.zeros((self.height, self.width), dtype=np.float32)
        terrain_noise = PerlinNoise(self.seed)
        cliff_noise = PerlinNoise(self.seed + 333)

        for y in range(self.height):
            y_norm = y / self.height
            for x in range(self.width):
                x_norm = x / self.width
                noise_val = terrain_noise.octave_noise(x * 0.006, y * 0.003, octaves=4, persistence=0.55)
                noise_val = max(0.0, (noise_val + 1.0) / 2.0)
                # Create sharp cliff edges by quantizing height
                cliff_edge = cliff_noise.octave_noise(x * 0.004, y * 0.002, octaves=3)
                cliff_edge = (cliff_edge + 1.0) / 2.0
                # Step function for cliff faces
                if cliff_edge > 0.55:
                    noise_val = min(1.0, noise_val * 1.8)
                elif cliff_edge < 0.45:
                    noise_val = noise_val * 0.3
                # Horizon fade
                horizon = smoothstep(clamp(1.0 - y_norm * 1.3, 0.0, 1.0))
                height_map[y, x] = noise_val * (0.3 + horizon * 0.7)

        height_map = (height_map - height_map.min()) / (height_map.max() - height_map.min() + 0.001)
        return height_map

    def _height_map_plains(self) -> np.ndarray:
        """Gentle rolling hills with distant mountains on the horizon."""
        height_map = np.zeros((self.height, self.width), dtype=np.float32)
        terrain_noise = PerlinNoise(self.seed)
        hill_noise = PerlinNoise(self.seed + 555)

        for y in range(self.height):
            y_norm = y / self.height
            for x in range(self.width):
                # Distant mountains (top 25%)
                mountain = terrain_noise.octave_noise(x * 0.004, y * 0.003, octaves=5, persistence=0.5)
                mountain = max(0.0, (mountain + 1.0) / 2.0)
                mountain_mask = smoothstep(clamp(1.0 - y_norm * 3.5, 0.0, 1.0))
                # Gentle rolling hills (lower portion)
                hills = hill_noise.octave_noise(x * 0.008, y * 0.006, octaves=2, persistence=0.3)
                hills = max(0.0, (hills + 1.0) / 2.0) * 0.25
                hill_mask = smoothstep(clamp(y_norm * 1.5 - 0.3, 0.0, 1.0))
                height_map[y, x] = mountain * mountain_mask * 0.8 + hills * hill_mask

        height_map = (height_map - height_map.min()) / (height_map.max() - height_map.min() + 0.001)
        return height_map

    def _height_map_cascades(self) -> np.ndarray:
        """Terraced mountain with multiple waterfall levels."""
        height_map = np.zeros((self.height, self.width), dtype=np.float32)
        terrain_noise = PerlinNoise(self.seed)

        for y in range(self.height):
            y_norm = y / self.height
            for x in range(self.width):
                noise_val = terrain_noise.octave_noise(x * 0.005, y * 0.004, octaves=4, persistence=0.5)
                noise_val = max(0.0, (noise_val + 1.0) / 2.0)
                # Create terrace steps by quantizing
                terraced = math.floor(noise_val * 5.0) / 5.0
                # Blend partial terrace with smooth for natural look
                noise_val = noise_val * 0.3 + terraced * 0.7
                # Horizon fade
                horizon = smoothstep(clamp(1.0 - y_norm * 1.4, 0.0, 1.0))
                height_map[y, x] = noise_val * (0.3 + horizon * 0.7)

        height_map = (height_map - height_map.min()) / (height_map.max() - height_map.min() + 0.001)
        return height_map

    def _height_map_world_map(self) -> np.ndarray:
        """Complex world map with varied terrain: mountains, plains, rivers, and lakes."""
        height_map = np.zeros((self.height, self.width), dtype=np.float32)
        
        # 1. Base terrain macro structure (continents/landmasses)
        macro_noise = PerlinNoise(self.seed)
        for y in range(self.height):
            y_norm = y / self.height
            for x in range(self.width):
                # Variety of biomes
                base = macro_noise.octave_noise(x * 0.002, y * 0.002, octaves=4, persistence=0.5)
                base = (base + 1.0) / 2.0
                
                # Push values to extremes for clear mountain/plain separation
                base = smoothstep(base)
                height_map[y, x] = base

        # 2. Add mountain ranges (ridges)
        mountain_noise = PerlinNoise(self.seed + 123)
        for y in range(self.height):
            for x in range(self.width):
                # Only add detail where base is high enough
                if height_map[y, x] > 0.4:
                    mtn = mountain_noise.octave_noise(x * 0.006, y * 0.006, octaves=5, persistence=0.55)
                    mtn = max(0.0, (mtn + 1.0) / 2.0)
                    
                    # Sharpen peaks
                    mtn = math.pow(mtn, 1.5)
                    
                    blend = smoothstep(clamp((height_map[y, x] - 0.4) * 3.0, 0.0, 1.0))
                    height_map[y, x] += mtn * 0.8 * blend

        # 3. Carve river valleys
        river_noise = PerlinNoise(self.seed + 456)
        # We will use this noise for the river path later too
        self._river_center_func_1 = lambda yy: 0.3 + river_noise.octave_noise(yy * 0.004, 0.0, octaves=3) * 0.15
        self._river_center_func_2 = lambda yy: 0.7 + river_noise.octave_noise(yy * 0.004 + 100, 0.0, octaves=3) * 0.15

        for y in range(self.height):
            # Two main river systems
            r1 = self._river_center_func_1(y) * self.width
            r2 = self._river_center_func_2(y) * self.width
            
            for x in range(self.width):
                # Carve river 1
                dist1 = abs(x - r1) / self.width
                valley1 = smoothstep(clamp(dist1 * 15.0, 0.0, 1.0))
                
                # Carve river 2
                dist2 = abs(x - r2) / self.width
                valley2 = smoothstep(clamp(dist2 * 15.0, 0.0, 1.0))
                
                height_map[y, x] *= valley1 * valley2

        height_map = (height_map - height_map.min()) / (height_map.max() - height_map.min() + 0.001)
        return height_map

    def create_paper_texture(self) -> Image.Image:
        """Create aged paper/scroll background texture."""
        img = Image.new('RGB', (self.width, self.height), self.paper_color)
        pixels = np.array(img, dtype=np.float32)

        # Add subtle paper grain
        grain_noise = PerlinNoise(self.seed + 1000)
        for y in range(self.height):
            for x in range(self.width):
                grain = grain_noise.noise(x * 0.05, y * 0.05) * 8
                pixels[y, x] += grain

        # Add age spots and stains
        stain_noise = PerlinNoise(self.seed + 2000)
        for y in range(self.height):
            for x in range(self.width):
                stain = stain_noise.octave_noise(x * 0.003, y * 0.003, octaves=3)
                if stain > 0.3:
                    darkness = (stain - 0.3) * 30
                    pixels[y, x] -= darkness

        # Darken edges (vignette)
        center_x, center_y = self.width / 2, self.height / 2
        max_dist = math.sqrt(center_x**2 + center_y**2)
        for y in range(self.height):
            for x in range(self.width):
                dist = math.sqrt((x - center_x)**2 + (y - center_y)**2)
                vignette = (dist / max_dist) ** 2 * 25
                pixels[y, x] -= vignette

        pixels = np.clip(pixels, 0, 255).astype(np.uint8)
        return Image.fromarray(pixels)

    def render_ink_wash(self, height_map: np.ndarray) -> Image.Image:
        """Render the main ink wash layers from height map with atmospheric perspective."""
        img = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))

        # Create distinct mountain layers - lighter in back, darker in front
        # This creates the classic "atmospheric perspective" of Chinese landscape painting
        layer_configs = [
            {"threshold": 0.65, "base_alpha": 35, "blur": 10, "lightness": 140},  # Far mountains (very light grey)
            {"threshold": 0.5, "base_alpha": 50, "blur": 7, "lightness": 100},    # Mid-far
            {"threshold": 0.38, "base_alpha": 70, "blur": 5, "lightness": 65},    # Middle
            {"threshold": 0.25, "base_alpha": 100, "blur": 3, "lightness": 35},   # Mid-close
            {"threshold": 0.12, "base_alpha": 160, "blur": 2, "lightness": 0},    # Close (darkest ink)
        ]

        for config in layer_configs:
            layer = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
            pixels = np.array(layer)

            threshold = config["threshold"]
            base_alpha = config["base_alpha"]
            lightness = config["lightness"]

            for y in range(self.height):
                # Vertical gradient - darker toward bottom of visible mountains
                y_factor = y / self.height

                for x in range(self.width):
                    h = height_map[y, x]
                    if h > threshold:
                        # Soft density falloff from threshold
                        density = min(1.0, (h - threshold) * 4)
                        density = smoothstep(density)

                        # Apply atmospheric lightening
                        r = min(255, self.ink_color[0] + lightness)
                        g = min(255, self.ink_color[1] + lightness)
                        b = min(255, self.ink_color[2] + lightness)

                        # Darker at the base of mountains
                        vertical_darkening = max(0, (y_factor - 0.3) * 0.3)
                        r = max(0, int(r - vertical_darkening * 30))
                        g = max(0, int(g - vertical_darkening * 30))
                        b = max(0, int(b - vertical_darkening * 30))

                        alpha = int(base_alpha * density)

                        # Add subtle texture variation
                        alpha = max(0, min(255, alpha + random.randint(-8, 8)))

                        pixels[y, x] = (r, g, b, alpha)

            layer = Image.fromarray(pixels)
            layer = layer.filter(ImageFilter.GaussianBlur(config["blur"]))
            img = Image.alpha_composite(img, layer)

        return img

    def render_contour_strokes(self, height_map: np.ndarray) -> Image.Image:
        """Render ink contour strokes - the 'cun' texturing strokes of Chinese painting."""
        img = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        # Compute gradients
        gradient_x = np.zeros_like(height_map)
        gradient_y = np.zeros_like(height_map)

        gradient_x[:, 1:] = height_map[:, 1:] - height_map[:, :-1]
        gradient_y[1:, :] = height_map[1:, :] - height_map[:-1, :]

        gradient_mag = np.sqrt(gradient_x**2 + gradient_y**2)

        # Draw strokes where gradient is strong (edges/ridges)
        stroke_threshold = 0.015

        for y in range(2, self.height - 2, 2):
            for x in range(2, self.width - 2, 2):
                if gradient_mag[y, x] > stroke_threshold and height_map[y, x] > 0.15:
                    # Skip randomly for organic feel
                    if random.random() > 0.4:
                        continue

                    # Stroke length varies with gradient
                    base_length = 8 + int(gradient_mag[y, x] * 150)
                    length = base_length + random.randint(-3, 5)

                    # Direction follows gradient (downslope) with some contour following
                    if gradient_mag[y, x] > 0.001:
                        # Mix between downslope and contour direction
                        mix = 0.6
                        dx = gradient_x[y, x] / gradient_mag[y, x] * (1 - mix) + \
                             (-gradient_y[y, x] / gradient_mag[y, x]) * mix
                        dy = gradient_y[y, x] / gradient_mag[y, x] * (1 - mix) + \
                             (gradient_x[y, x] / gradient_mag[y, x]) * mix
                    else:
                        continue

                    # Add randomness for brush-like quality
                    dx += random.uniform(-0.3, 0.3)
                    dy += random.uniform(-0.3, 0.3)

                    x2 = x + dx * length
                    y2 = y + dy * length

                    # Depth-based opacity (lighter in background)
                    depth = y / self.height
                    alpha = int(30 + depth * 120 + height_map[y, x] * 80)
                    alpha = min(220, alpha)

                    ink = (*self.ink_color, alpha)

                    # Stroke width varies
                    width = max(1, int(1 + depth * 2))
                    draw.line([(x, y), (x2, y2)], fill=ink, width=width)

        # Add ridge lines at peaks
        for y in range(5, self.height - 5, 4):
            for x in range(5, self.width - 5, 4):
                # Find local maxima (ridges)
                is_ridge = True
                for dy in [-3, 3]:
                    if 0 <= y + dy < self.height:
                        if height_map[y + dy, x] > height_map[y, x]:
                            is_ridge = False
                            break

                if is_ridge and height_map[y, x] > 0.3 and random.random() > 0.6:
                    # Draw horizontal ridge stroke
                    length = random.randint(10, 25)
                    depth = y / self.height
                    alpha = int(60 + depth * 100)
                    ink = (*self.ink_color, alpha)
                    draw.line([(x - length//2, y), (x + length//2, y)], fill=ink, width=1)

        return img

    def draw_pine_tree(self, draw: ImageDraw.Draw, x: int, y: int,
                       height: int, ink_alpha: int):
        """Draw a stylized pine tree silhouette."""
        ink = (*self.ink_color, ink_alpha)

        # Trunk
        trunk_width = max(2, height // 15)
        draw.line([(x, y), (x, y - height)], fill=ink, width=trunk_width)

        # Branches - asymmetric for natural look
        num_branches = height // 12
        for i in range(num_branches):
            branch_y = y - (i + 2) * (height // (num_branches + 2))

            # Left branch
            if random.random() > 0.2:
                length = random.randint(height // 6, height // 3)
                angle = random.uniform(-0.4, -0.1)
                bx = x - length
                by = branch_y + int(length * angle)
                draw.line([(x, branch_y), (bx, by)], fill=ink, width=max(1, trunk_width - 1))

                # Needle clusters
                for j in range(3):
                    nx = x - (j + 1) * length // 3
                    ny = branch_y + int((j + 1) * length // 3 * angle)
                    needle_len = random.randint(5, 12)
                    draw.line([(nx, ny), (nx - needle_len, ny - needle_len // 2)],
                             fill=ink, width=1)

            # Right branch
            if random.random() > 0.2:
                length = random.randint(height // 6, height // 3)
                angle = random.uniform(-0.4, -0.1)
                bx = x + length
                by = branch_y + int(length * angle)
                draw.line([(x, branch_y), (bx, by)], fill=ink, width=max(1, trunk_width - 1))

                for j in range(3):
                    nx = x + (j + 1) * length // 3
                    ny = branch_y + int((j + 1) * length // 3 * angle)
                    needle_len = random.randint(5, 12)
                    draw.line([(nx, ny), (nx + needle_len, ny - needle_len // 2)],
                             fill=ink, width=1)

    def draw_bamboo(self, draw: ImageDraw.Draw, x: int, y: int,
                    height: int, ink_alpha: int):
        """Draw stylized bamboo stalks."""
        ink = (*self.ink_color, ink_alpha)

        # Main stalk with segments
        segment_height = height // 6
        for i in range(6):
            seg_y = y - i * segment_height
            # Slight curve
            offset = int(math.sin(i * 0.5) * 3)
            draw.line([(x + offset, seg_y), (x + offset, seg_y - segment_height + 2)],
                     fill=ink, width=3)
            # Node
            draw.ellipse([(x + offset - 4, seg_y - 2), (x + offset + 4, seg_y + 2)],
                        outline=ink, width=1)

        # Leaves at nodes
        for i in range(2, 5):
            node_y = y - i * segment_height
            offset = int(math.sin(i * 0.5) * 3)

            # Leaf strokes
            for _ in range(random.randint(2, 4)):
                leaf_angle = random.uniform(-0.8, 0.8)
                leaf_len = random.randint(15, 30)
                side = random.choice([-1, 1])

                lx = x + offset + side * leaf_len
                ly = node_y - int(leaf_len * abs(leaf_angle))

                draw.line([(x + offset, node_y), (lx, ly)], fill=ink, width=2)

    def render_trees(self, height_map: np.ndarray) -> Image.Image:
        """Render sparse tree silhouettes."""
        img = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        # Place trees on ridge lines and slopes
        num_trees = random.randint(8, 20)

        for _ in range(num_trees):
            # Find suitable location
            attempts = 0
            while attempts < 50:
                x = random.randint(50, self.width - 50)
                y = random.randint(int(self.height * 0.2), int(self.height * 0.85))

                if height_map[y, x] > 0.3 and height_map[y, x] < 0.8:
                    # Check for ridge (local maximum)
                    local_max = True
                    for dy in range(-10, 11, 5):
                        if 0 <= y + dy < self.height:
                            if height_map[y + dy, x] > height_map[y, x]:
                                local_max = False
                                break

                    if local_max or random.random() > 0.7:
                        break
                attempts += 1

            if attempts >= 50:
                continue

            # Tree size based on depth (smaller in background)
            depth_factor = (y / self.height)
            tree_height = int(30 + depth_factor * 80)

            # Alpha based on depth (lighter in background - atmospheric perspective)
            alpha = int(100 + depth_factor * 155)

            # Choose tree type
            if random.random() > 0.3:
                self.draw_pine_tree(draw, x, y, tree_height, alpha)
            else:
                self.draw_bamboo(draw, x, y, tree_height, alpha)

        return img

    def draw_flying_birds(self, img: Image.Image) -> Image.Image:
        """Add distant flying birds - a classic element of Chinese landscape painting."""
        draw = ImageDraw.Draw(img)

        # Random number of bird groups
        num_groups = random.randint(1, 3)

        for _ in range(num_groups):
            # Position birds in upper portion of image
            group_x = random.randint(self.width // 4, 3 * self.width // 4)
            group_y = random.randint(int(self.height * 0.1), int(self.height * 0.4))

            # Birds in loose V or scattered formation
            num_birds = random.randint(3, 7)
            bird_size = random.randint(4, 8)

            # Depth-based alpha (lighter when higher/farther)
            depth = group_y / self.height
            alpha = int(80 + depth * 100)
            ink = (*self.ink_color, alpha)

            for i in range(num_birds):
                # Offset from group center
                bx = group_x + random.randint(-40, 40) + i * random.randint(-5, 15)
                by = group_y + random.randint(-20, 20) + abs(i - num_birds // 2) * random.randint(2, 8)

                # Simple bird shape - two curved strokes
                wing_span = bird_size + random.randint(-2, 2)

                # Left wing
                draw.arc(
                    [(bx - wing_span, by - wing_span // 2),
                     (bx, by + wing_span // 2)],
                    start=200, end=340,
                    fill=ink, width=1
                )
                # Right wing
                draw.arc(
                    [(bx, by - wing_span // 2),
                     (bx + wing_span, by + wing_span // 2)],
                    start=200, end=340,
                    fill=ink, width=1
                )

        return img

    def draw_distant_pagoda(self, draw: ImageDraw.Draw, x: int, y: int,
                            height: int, ink_alpha: int):
        """Draw a simple distant pagoda/pavilion silhouette."""
        ink = (*self.ink_color, ink_alpha)

        # Number of tiers
        tiers = random.randint(2, 4)
        tier_height = height // tiers

        current_y = y

        for i in range(tiers):
            # Each tier gets smaller going up
            tier_width = int((height // 2) * (1 - i * 0.2))

            # Roof (curved eaves)
            roof_points = [
                (x - tier_width, current_y),
                (x - tier_width // 2, current_y - tier_height // 3),
                (x, current_y - tier_height // 2),
                (x + tier_width // 2, current_y - tier_height // 3),
                (x + tier_width, current_y),
            ]

            # Draw roof outline
            for j in range(len(roof_points) - 1):
                draw.line([roof_points[j], roof_points[j + 1]], fill=ink, width=1)

            # Roof eave curves up at ends
            draw.line([(x - tier_width, current_y),
                      (x - tier_width - 3, current_y - 3)], fill=ink, width=1)
            draw.line([(x + tier_width, current_y),
                      (x + tier_width + 3, current_y - 3)], fill=ink, width=1)

            # Pillars/body
            body_width = tier_width // 2
            draw.line([(x - body_width, current_y),
                      (x - body_width, current_y + tier_height // 2)], fill=ink, width=1)
            draw.line([(x + body_width, current_y),
                      (x + body_width, current_y + tier_height // 2)], fill=ink, width=1)

            current_y -= tier_height

    def render_structures(self, height_map: np.ndarray) -> Image.Image:
        """Optionally render distant pagodas or pavilions."""
        img = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        base_chance = 0.4
        structure_attempts = int(1 + self.structure_density * 3)

        for _ in range(structure_attempts):
            if random.random() > (1.0 - base_chance * self.structure_density):
                for _ in range(20):
                    x = random.randint(self.width // 5, 4 * self.width // 5)
                    y = random.randint(int(self.height * 0.25), int(self.height * 0.5))

                    if height_map[y, x] > 0.35 and height_map[y, x] < 0.6:
                        depth = y / self.height
                        alpha = int(60 + depth * 120)
                        pagoda_height = int(25 + depth * 40)
                        self.draw_distant_pagoda(draw, x, y, pagoda_height, alpha)
                        break

        return img

    def add_color_wash(self, img: Image.Image) -> Image.Image:
        """Add subtle color tints in the style of colored ink wash."""
        pixels = np.array(img).astype(np.float32)

        # Create low-frequency color variation
        color_noise = PerlinNoise(self.seed + 5000)

        palette = self.palette_override if self.palette_override else [self.wash_tint]

        for y in range(self.height):
            for x in range(self.width):
                # Get color variation
                color_factor = (color_noise.octave_noise(x * 0.002, y * 0.002, octaves=2) + 1) / 2

                # Blend with wash tint
                blend = 0.05 + color_factor * 0.12
                tint = palette[int(color_factor * (len(palette) - 1))]
                for c in range(3):
                    pixels[y, x, c] = pixels[y, x, c] * (1 - blend) + tint[c] * blend

        pixels = np.clip(pixels, 0, 255).astype(np.uint8)
        return Image.fromarray(pixels)

    def apply_time_of_day(self, img: Image.Image) -> Image.Image:
        """Apply time-of-day mood grading."""
        preset = self.TIME_OF_DAY_PRESETS.get(self.time_of_day, self.TIME_OF_DAY_PRESETS["none"])
        pixels = np.array(img).astype(np.float32)

        brightness = preset["brightness"]
        contrast = preset["contrast"]
        warmth = preset["warmth"]

        # Contrast adjustment around mid-gray
        pixels = (pixels - 128.0) * contrast + 128.0
        pixels = pixels * brightness

        # Warmth: shift red/blue channels
        pixels[:, :, 0] += 20 * warmth
        pixels[:, :, 2] -= 20 * warmth

        pixels = np.clip(pixels, 0, 255)
        return Image.fromarray(pixels.astype(np.uint8))

    def apply_weather(self, img: Image.Image) -> Image.Image:
        """Apply weather effects like rain, snow, or storm."""
        if self.weather == "none":
            return img

        layer = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(layer)

        if self.weather == "rain":
            num_streaks = int(self.width * self.height / 1800)
            for _ in range(num_streaks):
                x = random.randint(0, self.width)
                y = random.randint(0, self.height)
                length = random.randint(10, 25)
                angle = random.uniform(-0.6, -0.2)
                x2 = x + int(length * math.cos(angle))
                y2 = y + int(length * math.sin(angle))
                draw.line([(x, y), (x2, y2)], fill=(180, 180, 190, 80), width=1)

        elif self.weather == "snow":
            num_flakes = int(self.width * self.height / 900)
            for _ in range(num_flakes):
                x = random.randint(0, self.width)
                y = random.randint(0, self.height)
                size = random.randint(1, 3)
                draw.ellipse([(x, y), (x + size, y + size)], fill=(240, 240, 240, 140))

        elif self.weather == "storm":
            # Dark cloud wash
            draw.rectangle([(0, 0), (self.width, self.height)], fill=(20, 20, 30, 60))
            if random.random() > 0.5:
                # Lightning strike
                x = random.randint(self.width // 4, 3 * self.width // 4)
                points = [(x, 0)]
                for _ in range(6):
                    x += random.randint(-20, 20)
                    y = points[-1][1] + random.randint(40, 80)
                    points.append((x, min(y, self.height)))
                draw.line(points, fill=(240, 240, 255, 200), width=2)

        layer = layer.filter(ImageFilter.GaussianBlur(1))
        return Image.alpha_composite(img.convert('RGBA'), layer)

    def apply_celestial_glow(self, img: Image.Image, height_map: np.ndarray) -> Image.Image:
        """Add a celestial glow around a peak for breakthrough/heavenly scenes."""
        if self.celestial_glow <= 0.0:
            return img

        glow_layer = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        pixels = np.array(glow_layer)

        # Find brightest peak location
        peak_index = np.unravel_index(np.argmax(height_map), height_map.shape)
        py, px = int(peak_index[0]), int(peak_index[1])
        radius = int(min(self.width, self.height) * (0.18 + 0.2 * self.celestial_glow))

        for y in range(self.height):
            for x in range(self.width):
                dist = math.sqrt((x - px) ** 2 + (y - py) ** 2)
                if dist < radius:
                    strength = 1.0 - (dist / radius)
                    strength = smoothstep(strength) * self.celestial_glow
                    alpha = int(180 * strength)
                    pixels[y, x] = (240, 210, 140, alpha)

        glow_layer = Image.fromarray(pixels)
        glow_layer = glow_layer.filter(ImageFilter.GaussianBlur(12))
        return Image.alpha_composite(img.convert('RGBA'), glow_layer)

    def add_mist(self, img: Image.Image, height_map: np.ndarray) -> Image.Image:
        """Add atmospheric mist between mountain layers - key to the ink wash style."""
        mist_layer = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        pixels = np.array(mist_layer)

        mist_noise = PerlinNoise(self.seed + 3000)

        # Mist bands at transition zones between mountain layers
        mist_bands = [
            {"height": 0.65, "thickness": 0.15, "intensity": 0.9},   # Upper mist
            {"height": 0.45, "thickness": 0.12, "intensity": 1.0},   # Middle mist (most prominent)
            {"height": 0.28, "thickness": 0.1, "intensity": 0.7},    # Lower mist
        ]

        for band in mist_bands:
            mist_h = band["height"]
            thickness = band["thickness"]
            intensity = band["intensity"]

            for y in range(self.height):
                # Mist tends to settle in horizontal bands
                y_norm = y / self.height

                for x in range(self.width):
                    h = height_map[y, x]

                    # Mist appears where height is near the band level
                    dist_from_band = abs(h - mist_h)
                    if dist_from_band < thickness:
                        mist_strength = 1 - (dist_from_band / thickness)
                        mist_strength = smoothstep(mist_strength)

                        # Noise creates wispy, cloud-like patterns
                        noise_val = mist_noise.octave_noise(
                            x * 0.006 + y * 0.001,
                            y * 0.004,
                            octaves=4,
                            persistence=0.6
                        )
                        noise_val = (noise_val + 1) / 2

                        # Mist is stronger in valleys (lower height values locally)
                        if noise_val > 0.3:
                            alpha = int(mist_strength * (noise_val - 0.3) * 2 * intensity * 140)
                            alpha = min(180, alpha)

                            # Mist color - slightly warm white
                            mr = min(255, self.paper_color[0] + 5)
                            mg = min(255, self.paper_color[1] + 3)
                            mb = min(255, self.paper_color[2])

                            # Blend with existing
                            existing_alpha = pixels[y, x, 3]
                            if alpha > existing_alpha:
                                pixels[y, x] = (mr, mg, mb, alpha)

        mist_layer = Image.fromarray(pixels)
        # Heavy blur for soft, ethereal effect
        mist_layer = mist_layer.filter(ImageFilter.GaussianBlur(20))

        return Image.alpha_composite(img.convert('RGBA'), mist_layer)

    def render_river(self, height_map: np.ndarray) -> Image.Image:
        """Render a winding river/stream through the landscape."""
        img = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        river_noise = PerlinNoise(self.seed + 888)

        if hasattr(self, '_river_center_func'):
            center_func = self._river_center_func
        else:
            # Default river path based on noise
            center_func = lambda yy: 0.5 + river_noise.octave_noise(yy * 0.005, 0.0, octaves=3) * 0.2

        # Draw river from top to bottom
        prev_x = None
        prev_y = None
        for y in range(0, self.height, 2):
            y_norm = y / self.height
            center_x = center_func(y) * self.width

            # River width varies
            width_variation = river_noise.noise(y * 0.01, 100.0)
            river_width = int(12 + y_norm * 25 + width_variation * 8)

            # Water color - semi-transparent blue-grey ink wash
            depth_alpha = int(40 + y_norm * 60)
            water_color = (70, 85, 105, depth_alpha)

            x1 = int(center_x - river_width / 2)
            x2 = int(center_x + river_width / 2)
            draw.rectangle([(x1, y), (x2, y + 2)], fill=water_color)

            # Ripple lines
            if random.random() > 0.7:
                ripple_alpha = int(depth_alpha * 0.6)
                ripple_ink = (*self.ink_color, ripple_alpha)
                ripple_x = int(center_x + random.randint(-river_width // 3, river_width // 3))
                ripple_len = random.randint(5, river_width // 2)
                draw.line([(ripple_x, y), (ripple_x + ripple_len, y)],
                         fill=ripple_ink, width=1)

            prev_x = int(center_x)
            prev_y = y

        img = img.filter(ImageFilter.GaussianBlur(3))
        return img

    def render_lake(self, height_map: np.ndarray) -> Image.Image:
        """Render a calm lake surface in lower portions of the image."""
        img = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        pixels = np.array(img)

        ripple_noise = PerlinNoise(self.seed + 999)

        for y in range(self.height):
            y_norm = y / self.height
            for x in range(self.width):
                # Lake appears where terrain is low (below shoreline)
                if height_map[y, x] < 0.2 and y_norm > 0.45:
                    # Distance from shore affects water appearance
                    shore_dist = 0.2 - height_map[y, x]
                    water_strength = smoothstep(clamp(shore_dist * 8.0, 0.0, 1.0))

                    # Ripple pattern
                    ripple = ripple_noise.noise(x * 0.02, y * 0.008)
                    ripple = (ripple + 1.0) / 2.0

                    alpha = int(water_strength * (50 + ripple * 30))
                    # Reflection gets darker further from shore
                    r = int(60 + ripple * 20)
                    g = int(75 + ripple * 15)
                    b = int(100 + ripple * 20)

                    pixels[y, x] = (r, g, b, alpha)

        lake_layer = Image.fromarray(pixels)
        lake_layer = lake_layer.filter(ImageFilter.GaussianBlur(4))

        # Add horizontal ripple lines
        draw = ImageDraw.Draw(lake_layer)
        for y in range(int(self.height * 0.5), self.height, random.randint(6, 12)):
            for x_start in range(0, self.width, random.randint(30, 80)):
                if height_map[min(y, self.height - 1), min(x_start, self.width - 1)] < 0.18:
                    ripple_len = random.randint(15, 50)
                    alpha = random.randint(30, 70)
                    draw.line(
                        [(x_start, y), (x_start + ripple_len, y)],
                        fill=(*self.ink_color, alpha), width=1
                    )

        return lake_layer

    def render_waterfall(self, height_map: np.ndarray) -> Image.Image:
        """Render waterfall streaks at terrace edges (for cascades terrain)."""
        img = Image.new('RGBA', (self.width, self.height), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        # Find steep vertical drops in the height map
        for x in range(10, self.width - 10, 8):
            for y in range(10, self.height - 10, 4):
                if y + 5 < self.height:
                    drop = height_map[y, x] - height_map[y + 5, x]
                    if drop > 0.08:
                        # Steep drop - draw waterfall streak
                        streak_height = int(drop * 200) + random.randint(10, 30)
                        alpha = int(60 + drop * 300)
                        alpha = min(180, alpha)

                        # White-blue water streak
                        water_color = (180, 195, 210, alpha)
                        streak_width = random.randint(1, 3)

                        x_wobble = random.randint(-3, 3)
                        draw.line(
                            [(x + x_wobble, y), (x + x_wobble + random.randint(-2, 2), y + streak_height)],
                            fill=water_color, width=streak_width
                        )

                        # Spray at base
                        if random.random() > 0.5:
                            spray_y = y + streak_height
                            for _ in range(3):
                                sx = x + random.randint(-8, 8)
                                sy = spray_y + random.randint(-3, 5)
                                draw.ellipse(
                                    [(sx, sy), (sx + 3, sy + 2)],
                                    fill=(200, 210, 220, alpha // 2)
                                )

        img = img.filter(ImageFilter.GaussianBlur(2))
        return img

    def draw_calligraphy_mark(self, img: Image.Image) -> Image.Image:
        """Add a simple seal/stamp mark in the corner."""
        draw = ImageDraw.Draw(img)

        # Red seal stamp
        seal_size = min(self.width, self.height) // 12
        margin = seal_size // 2

        # Position in corner
        positions = [
            (self.width - margin - seal_size, margin),  # Top right
            (margin, self.height - margin - seal_size),  # Bottom left
        ]
        sx, sy = random.choice(positions)

        # Draw seal border
        seal_color = (180, 50, 40, 200)  # Traditional red
        draw.rectangle(
            [(sx, sy), (sx + seal_size, sy + seal_size)],
            outline=seal_color,
            width=3
        )

        # Simple decorative lines inside (mimicking characters)
        for i in range(3):
            y_pos = sy + seal_size // 4 + i * seal_size // 4
            x_start = sx + seal_size // 5
            x_end = sx + seal_size * 4 // 5
            draw.line([(x_start, y_pos), (x_end, y_pos)], fill=seal_color, width=2)

        return img

    def generate(self) -> Image.Image:
        """Generate complete ink wash landscape image."""
        print(f"Generating with seed: {self.seed}")

        # Generate base layers
        print("  Creating paper texture...")
        paper = self.create_paper_texture()

        print("  Generating height map...")
        height_map = self.generate_height_map()

        print("  Rendering ink wash layers...")
        ink_wash = self.render_ink_wash(height_map)

        print("  Drawing contour strokes...")
        contours = self.render_contour_strokes(height_map)

        print("  Placing trees...")
        trees = self.render_trees(height_map)

        print("  Adding structures...")
        structures = self.render_structures(height_map)

        # Composite layers
        print("  Compositing layers...")
        result = paper.convert('RGBA')
        result = Image.alpha_composite(result, ink_wash)
        result = Image.alpha_composite(result, contours)
        result = Image.alpha_composite(result, structures)
        result = Image.alpha_composite(result, trees)

        # Water features based on terrain type
        if self.terrain == "river_valley":
            print("  Rendering river...")
            river = self.render_river(height_map)
            result = Image.alpha_composite(result, river)
        elif self.terrain == "lakeside":
            print("  Rendering lake...")
            lake = self.render_lake(height_map)
            result = Image.alpha_composite(result, lake)
            print("  Rendering waterfalls...")
            waterfalls = self.render_waterfall(height_map)
            result = Image.alpha_composite(result, waterfalls)
        elif self.terrain == "world_map":
            print("  Rendering world map features...")
            # Render multiple rivers
            self._river_center_func = self._river_center_func_1
            river1 = self.render_river(height_map)
            result = Image.alpha_composite(result, river1)
            
            self._river_center_func = self._river_center_func_2
            river2 = self.render_river(height_map)
            result = Image.alpha_composite(result, river2)
            
            # Render lakes in low areas
            print("  Rendering lakes...")
            lake = self.render_lake(height_map)
            result = Image.alpha_composite(result, lake)
            
            # Render waterfalls on steep slopes
            print("  Rendering waterfalls...")
            waterfalls = self.render_waterfall(height_map)
            result = Image.alpha_composite(result, waterfalls)

        # Add atmospheric effects
        print("  Adding mist...")
        result = self.add_mist(result, height_map)

        # Add flying birds
        print("  Adding birds...")
        result = self.draw_flying_birds(result)

        # Add color wash
        print("  Applying color wash...")
        result = self.add_color_wash(result)

        # Time-of-day grading
        if self.time_of_day != "none":
            print("  Applying time-of-day grading...")
            result = self.apply_time_of_day(result)

        # Celestial glow
        if self.celestial_glow > 0.0:
            print("  Applying celestial glow...")
            result = self.apply_celestial_glow(result, height_map)

        # Weather overlay
        if self.weather != "none":
            print("  Applying weather effects...")
            result = self.apply_weather(result)

        # Add seal (optional)
        if self.show_seal:
            result = self.draw_calligraphy_mark(result)

        # Final adjustments
        result = result.convert('RGB')

        # Slight final blur for cohesion
        result = result.filter(ImageFilter.GaussianBlur(0.5))

        return result


def main():
    parser = argparse.ArgumentParser(
        description="Generate procedural ink-wash landscape images"
    )
    parser.add_argument(
        "-o", "--output",
        type=str,
        default="ink_wash_landscape.png",
        help="Output file path"
    )
    parser.add_argument(
        "-W", "--width",
        type=int,
        default=1024,
        help="Image width (default: 1024)"
    )
    parser.add_argument(
        "-H", "--height",
        type=int,
        default=768,
        help="Image height (default: 768)"
    )
    parser.add_argument(
        "-s", "--seed",
        type=int,
        default=None,
        help="Random seed for reproducibility"
    )
    parser.add_argument(
        "-n", "--count",
        type=int,
        default=1,
        help="Number of images to generate"
    )
    parser.add_argument(
        "--element",
        type=str,
        default="default",
        help="Element palette: metal|wood|water|fire|earth|default"
    )
    parser.add_argument(
        "--time-of-day",
        type=str,
        default="none",
        help="Time of day: dawn|day|dusk|night|none"
    )
    parser.add_argument(
        "--weather",
        type=str,
        default="none",
        help="Weather: rain|snow|storm|none"
    )
    parser.add_argument(
        "--structure-density",
        type=float,
        default=1.0,
        help="Structure density multiplier (0.0-2.0)"
    )
    parser.add_argument(
        "--celestial-glow",
        type=float,
        default=0.0,
        help="Celestial glow intensity (0.0-1.0)"
    )
    parser.add_argument(
        "--no-seal",
        action="store_true",
        help="Disable the red seal/stamp mark"
    )
    parser.add_argument(
        "--terrain",
        type=str,
        default="mountains",
        help="Terrain type: mountains|river_valley|lakeside|cliffs|plains|cascades"
    )

    args = parser.parse_args()

    output_path = Path(args.output)

    for i in range(args.count):
        seed = args.seed + i if args.seed is not None else None

        generator = InkWashGenerator(
            width=args.width,
            height=args.height,
            seed=seed,
            element_palette=args.element,
            time_of_day=args.time_of_day,
            weather=args.weather,
            structure_density=args.structure_density,
            celestial_glow=args.celestial_glow,
            show_seal=not args.no_seal,
            terrain=args.terrain,
        )

        img = generator.generate()

        if args.count > 1:
            out_file = output_path.parent / f"{output_path.stem}_{i+1}{output_path.suffix}"
        else:
            out_file = output_path

        img.save(out_file, quality=95)
        print(f"Saved: {out_file}")


if __name__ == "__main__":
    main()
