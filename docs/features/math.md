### aid math eval
```
  aid math eval <EXPRESSION>  Evaluates a math expression

-----input-----
aid math eval "sin(pi / 2) * 8 ^ e"

-----output-----
285.0054081607272
```

### aid math plot
```
  aid math plot <EXPRESSION> Plots a math expression
              --start <START_X>   Start x coord.
              --end <END_X>       End x coord.
            -s, --step <STEP_X>     x step size.

-----input-----
aid math plot --start -20 --end 20 --step 0.5 "x * sin(1 - x)"

-----output-----
expression: x * sin(1 - x)
X range: [-20, 20]
Y range: [-19.43818, 18.049082]
                        |                      *
    *                   |                     **
   ***                  |                     **
   * *                  |                    ***
   * *                  |             **     * **
   * *     **           |             ***    *  *
   * *     ***          |             * *    *  *
  **  *   ** *          |     ***    *  *    *  *
  *   *   *  *    ***   |     * *    *  *    *  *
  *   *   *  **   * **  |    **  *   *  **   *  *
--*---*---*---*--**--******-**---*---*---*--*---*-
  *   *  **   *  *      | ***    *  *    *  *   **
  *   *  *    ****      |         * *    *  *
  *    * *     **       |         * *    *  *
 **    * *              |         ***    ** *
 *     ***              |                 ***
 *     **               |                 **
 *                      |                 **
**                      |
**                      |
```