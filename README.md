# Loaded String Simulation

Simple physics simulation of loaded string using verlet integration and nearest-neighbor interaction to dictate forces.

## Outline of Settings
1. Basic Settings
    - Reset: resets the dynamics of the system but doesn't affect the display settings.
    - Masses: alters the number of simulated masses. The number can be directly altered with the slider, by clicking on the number of masses or using the increment and decrement buttons.
    - Delta: alters the timestep of the simulation. Ranges from 0.001 to 0.75 (inclusive). Larger timesteps run faster than smaller timesteps. Altering the timestep causes the simulation to reset to avoid odd behavior.
    - Animate: starts or stops the animation of the plot.
    - Step: continues the simulation by a single time-step. Even if the simulation run-time is clamped, step will still function.

2. Display Settings
    - Mass radius: the display size of the masses on the string.
    - String length: the display length of the string.
    - Mass color: the color of the masses on the string.
    - String color: the color of the string itself.
    - Boundary color: the color of the boundary.
    - Boundary style: the type of boundary used by the simulation (lines, fixed masses, none). Purely display (no physical effect on the simulation).
    - Windowed: the side panel is its own window or part of the main window.

3. Misc State Settings
    - Initial displacement: sets the max point for the harmonic, parabolic, and pluck functions.
    - Harmonic state: sets the harmonic state for the harmonic function.
    - Harmonic: initializes the system of the given size in a harmonic state.
    - Parabolic: initializes the system of the given size in a parabolic state.
    - Pluck: initializes the system of the given size in a pluck state.

4. Clamped Settings
    - Max time: maximum time the simulation will run for while clamped.
    - Clamped: whether the simulation runtime is clamped or not.

5. Mass Positions
Displays the position of each mass on the loaded string and allows you to modify the position of each mass individually, even while the simulation is running.

6. Menu Options
Under file, there are three options: 
    1. Quit: Closes the window for the native windows binary. Doesn't display any effect for the Web application.
    2. Toggle Window: Another option to toggle the windowed side panel.
    3. Full reset: Fully resets the simulation (system dynamics, display settings, clamped settings, etc.)
