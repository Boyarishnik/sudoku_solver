use std::{
    cell::{RefCell, RefMut},
    fmt::{Debug, Display},
};

#[derive(Debug, Clone)]
pub struct FieldCell {
    value: Option<u8>,
}

impl FieldCell {
    pub fn new() -> Self {
        Self {
            value: None,
        }
    }

    pub fn value(&self) -> Option<u8> {
        self.value
    }

    fn set_value(&mut self, value: u8) -> Result<(), ()> {
        if 0 < value && value < 10 {
            self.value = Some(value);
            return Ok(());
        }
        Err(())
    }

    fn delete_value(&mut self) {
        self.value = None;
    }
}

pub struct Field {
    pub cells: [[RefCell<FieldCell>; 9]; 9],
}

impl Field {
    pub fn new() -> Self {
        let row: [RefCell<FieldCell>; 9] = (0..9)
            .map(|_| RefCell::new(FieldCell::new()))
            .collect::<Vec<RefCell<FieldCell>>>()
            .try_into()
            .unwrap();

        let cells: [[RefCell<FieldCell>; 9]; 9] = (0..9)
            .map(|_| row.clone())
            .collect::<Vec<[RefCell<FieldCell>; 9]>>()
            .try_into()
            .unwrap();

        Self { cells }
    }

    pub fn try_push(&mut self, position: (usize, usize), number: u8) -> Result<(), ()> {
        if self.get_possible_vals(position).contains(&number) {
            let mut cell = self.cells[position.0][position.1].borrow_mut();
            cell.set_value(number)?;
        }

        Ok(())
    }

    fn get_next_empty(&self, position: (usize, usize)) -> Option<(usize, usize)> {
        for i in position.1..9 {
            if self.cells[position.0][i]
                .borrow()
                .value()
                .is_none()
            {
                return Some((position.0, i));
            }
        }

        for i in position.0 + 1..9 {
            for j in 0..9 {
                if self.cells[i][j].borrow().value().is_none() {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn get_3x3_square(&self, position: (usize, usize)) -> Vec<Vec<RefMut<FieldCell>>> {
        let row = position.0 / 3 * 3;
        let col = position.1 / 3 * 3;

        (row..row + 3)
            .map(|x| {
                (col..col + 3)
                    .map(|y| self.cells[x][y].borrow_mut())
                    .collect()
            })
            .collect()
    }

    pub fn cancel_insertion(&mut self, position: (usize, usize)) -> Result<(), ()> {
        self.cells[position.0][position.1].borrow_mut().delete_value();

        Ok(())
    }

    pub fn solve(&mut self) -> Result<(), ()> {
        self.rec_solve((0, 0))
    }

    fn rec_solve(&mut self, position: (usize, usize)) -> Result<(), ()> {
        if let Some(cell_pos) = self.get_next_empty(position) {
            let vals = self.get_possible_vals(cell_pos);

            for i in vals {
                if let Ok(_) = self.try_push(cell_pos, i) {
                    match self.rec_solve((position.0, position.1 + 1)) {
                        Ok(_) => return Ok(()),
                        Err(_) => self.cancel_insertion(cell_pos).unwrap(),
                    }
                }
            }

            return Err(())
        }

        Ok(())
    }

    fn get_possible_vals(&self, position: (usize, usize)) -> Vec<u8> {
        let mut res: Vec<u8> = (0..10).collect();

        for i in 0..9 {
            {
                let cell = self.cells[i][position.1].borrow();
                if let Some(val) = cell.value() {
                    if res.contains(&val) {
                        res.retain(|x| *x != val);
                    }
                }
            }

            {
                let cell = self.cells[position.0][i].borrow();
                if let Some(val) = cell.value() {
                    if res.contains(&val) {
                        res.retain(|x| *x != val);
                    }
                }
            }
        }

        for i in self.get_3x3_square(position) {
            for j in i {
                if let Some(val) = j.value() {
                    if res.contains(&val) {
                        res.retain(|x| *x != val);
                    }
                }
            }
        }

        res
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.cells {
            writeln!(f, "{:?}", i).unwrap();
        }
        Ok(())
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.cells {
            writeln!(
                f,
                "{}",
                i.iter()
                    .map(|x| match x.borrow().value() {
                        Some(num) => format!("{} ", (num + b'0') as char),
                        None => String::from("_ "),
                    })
                    .collect::<String>()
            )
            .unwrap();
        }
        Ok(())
    }
}
