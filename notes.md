# Notes

https://stackoverflow.com/questions/62286695/is-there-a-simple-ish-algorithm-for-drawing-force-directed-graphs
* gravity toward center of drawing
* repulsion between graph nodes
* attraction to fixed distance along edge

https://cs.brown.edu/people/rtamassi/gdhandbook/chapters/force-directed.pdf

* force exerted by spring is c_1 * log(d/c_2)
* nonadjascent vertices repel like c_3/d^2
* algorithm:
** place vertices randomly
** for M iterations:
*** calculate force F_i on each vertex v_i
*** move v_i by c_4 * F_i
* defaults are c_1 = 2, c_2 = 1, c_3 = 1, c_4 = 0.1, M = 100
