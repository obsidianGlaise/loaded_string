# Loaded String

Simple physics simulation of loaded string using verlet integration and nearest-neighbor interaction to dictate forces.

## Outline of Functions
1. Basic function
You can directly manipulate the position of each mass. You can also directly alter the number of masses on the string.

2. Reset
You can reset the system (size, masses, accelerations, time, animate is set to false).
Things that (currently) don't reset: Delta and Clamped Time.

3. Delta
You can alter the timestep of the system. A smaller timestep causes the simulation to run more slowly.
After altering the timestep, it is recommended to reset the system as the simulation isn't accurate after the delta is adjusted.

4. Increment and Decrement
Increment and decrement the number of masses by 1. Clamped to 1..300 (inclusive of both sides)

5. Harmonic State (WIP)
Sets the position of each mass according to the first fundamental harmonic. 
Initial displacement sets the peak. Clicking start will set each mass to their position, set time = 0, and animate = false.

6. Mass radius
Purely for display purpose. Adjust to better see the dynamics of each individual mass.

7. Step
Stop animating if animating and move by a single time step (delta). Even if clamped, step still works (though animate does not).