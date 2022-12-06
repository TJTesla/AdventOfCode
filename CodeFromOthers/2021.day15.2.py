input = open("../input/year2021/day15.txt",'r').read()
gridsize = 100

lines = input.split('\n')
grid = []
for line in lines:
    line_list = list(int(x) for x in list(line))
    grid.append(line_list)

risks = []            # grid of accumnulated risk values moving from the "top" (for the top half of the diamond)
                      # or "bottom" (for the bottom half of the diamond) to that point
for i in range(gridsize):
    row = []
    for j in range(gridsize):
        row.append(0)          # fill with 0's to start
    risks.append(row)

def get_neighbors(point, where):   # where = 'above', 'below', or 'both'
    neighbors = []
    above = True if (where == 'above' or where == 'both') else False
    below = True if (where == 'below' or where == 'both') else False
    if (p[0] > 0) and above:  # point is not on the top edge in original grid
        neighbors.append([p[0] - 1, p[1]])
    if (p[1] > 0) and above:  # point is not on the left edge in the original grid
        neighbors.append([p[0], p[1] - 1])
    if (p[0] < (gridsize - 1)) and below:  # point is not on the bottom edge in the original grid
        neighbors.append([p[0] + 1, p[1]])
    if (p[1] < (gridsize - 1)) and below:  # point is not on the right edge in the original grid
        neighbors.append([p[0], p[1] + 1])
    return neighbors


# top half of diamond
for i in range(1, gridsize):  #start at 2nd row of diamond
    top_row_points = []
    for j in range(i+1):   # that row has i+1 points
        x = j
        y = i-j
        top_row_points.append([y, x])
    for p in top_row_points:
        neighbors = get_neighbors(p, 'above')
        neighbor_risks = []
        for neighbor in neighbors:
            neighbor_risks.append(risks[neighbor[0]][neighbor[1]])
        my_risk = grid[p[0]][p[1]] + min(neighbor_risks)
        risks[p[0]][p[1]] = my_risk


# bottom half of diamond
risks[gridsize-1][gridsize-1] = grid[gridsize-1][gridsize-1]  # Include the risk of the end point, since we will move to it
for i in range(1, gridsize-1):   # only go to the row below the middle of the diamond
    bottom_row_points = []
    for j in range(i+1):   # that row has i+1 points
        x = gridsize - j - 1
        y = gridsize - (i - j) - 1
        bottom_row_points.append([y, x])
    for p in bottom_row_points:
        neighbors = get_neighbors(p, 'below')
        neighbor_risks = []
        for neighbor in neighbors:
            neighbor_risks.append(risks[neighbor[0]][neighbor[1]])
        my_risk = grid[p[0]][p[1]] + min(neighbor_risks)
        risks[p[0]][p[1]] = my_risk

# Finally, combine top and bottom half
middle_row_risks = []
for p in top_row_points:
    neighbors = get_neighbors(p, 'below')
    neighbor_risks = []
    for neighbor in neighbors:
        neighbor_risks.append(risks[neighbor[0]][neighbor[1]])
    my_risk = risks[p[0]][p[1]] + min(neighbor_risks)
    middle_row_risks.append(my_risk)

min_risk = min(middle_row_risks)
print('min risk = {}'.format(min_risk))
