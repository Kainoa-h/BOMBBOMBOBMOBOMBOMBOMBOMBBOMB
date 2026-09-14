impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (blx1, bly1, trx1, try1) = (rec1[0], rec1[1], rec1[2], rec1[3]);
        let (blx2, bly2, trx2, try2) = (rec2[0], rec2[1], rec2[2], rec2[3]);
        let x_overlap = blx1 < trx2 && blx2 < trx1;
        let y_overlap = bly1 < try2 && bly2 < try1;
        x_overlap && y_overlap
    }
}

struct Solution {}
