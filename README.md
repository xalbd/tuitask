# TUI-Task

Terminal-based task manager written in Rust using a PostgreSQL database.

## Features

- Intuitive keyboard-based controls with context-aware help for keybinds
- Create tasks and easily edit their names/due dates
- Create categories and sort tasks into them
- View tasks in an upcoming view or grouped by category
- View completed tasks and uncomplete them easily
- Support for repeating tasks
- Automatic creation of a set of individually view/editable repeating tasks (e.g. Homework 1, 2, ...)
- Support for "Week 0 Day 1" dating to align with school schedules
- Responsive UI with async connection to Postgres database
