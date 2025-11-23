# Input CSV file
datafile = "result.txt"

# Output image (HD resolution)
set terminal pngcairo size 1920,1080 enhanced font "Arial,14"
set output "output.png"

# CSV separator
set datafile separator ","

# Labels
set title "Plot of CSV Data"
set xlabel "Column 1"
set ylabel "Column 2"

# Grid
set grid

# Draw horizontal line at y=0
set arrow from graph 0, first 0 to graph 1, first 0 nohead lw 2 lc rgb "black"

# Plot
plot datafile using 1:2 with linespoints lt rgb "blue" lw 2 pt 7 title "Data"
