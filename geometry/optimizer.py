"""
FrostGuard Geometry Optimizer
Optimal heater placement and coverage formula for maximum freeze protection.

Bottom (6 o'clock) and 5/7 o'clock positions are prioritized.
"""

import math

def optimal_pitch(diameter_mm: float) -> float:
    """Spiral pitch recommendation (mm). Straight for small pipes."""
    if diameter_mm <= 50:
        return 0.0  # Straight run preferred
    return 0.25 * diameter_mm

def heat_coverage_watts(power_w: float, efficiency: float, diameter_m: float, length_m: float) -> float:
    """Effective heat coverage formula."""
    return (power_w * efficiency) / (math.pi * diameter_m * length_m)

def position_priority(clock_position: int) -> float:
    """Priority multiplier by clock position around the pipe."""
    if clock_position == 6:          # Bottom
        return 1.15
    if clock_position in (5, 7):     # Offset bottom
        return 1.08
    return 1.0

if __name__ == "__main__":
    print("FrostGuard Geometry Optimizer loaded")
    print(f"Recommended pitch for 25mm pipe: {optimal_pitch(25)} mm (straight)")
    print(f"Recommended pitch for 75mm pipe: {optimal_pitch(75):.1f} mm")
