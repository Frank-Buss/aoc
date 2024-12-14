#!/usr/bin/bash

# needs this:
# sudo apt-get install pandoc texlive-latex-base texlive-fonts-recommended texlive-fonts-extra texlive-latex-extra

pandoc -s -V papersize:a4 day13.md -o day13.tex
pdflatex day13.tex
xdg-open day13.pdf
rm -f day13.log day13.tex day13.aux
