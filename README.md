[![Build Status](https://travis-ci.com/entscheidungsproblem/K-means.svg?branch=master)](https://travis-ci.com/entscheidungsproblem/K-means)

# K-means
I decided to work on this project to help me learn Rust and to create a precise color extraction algorithm that I had playing with in Python.

This project takes a different mindset then other color extraction programs as it focuses on accuracy/precision over speed. It uses LCH color space and CIELAB Delta E distances to calculate color distance as a human eye perceives it in order to create highly accurate color clusters. Special thanks to [http://www.brucelindbloom.com/](http://www.brucelindbloom.com/) for the useful resources!
