use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_curve_testing {
    use std::convert::TryInto;

    use anchor_lang::{prelude::borsh::to_vec, solana_program::log::sol_log_compute_units};

    use super::*;

    /*
    options
    1. resize every time
    2. resize periodically

    when u resize
    1. resize the account (delete and re-issue with more space)
    2. resize the array only ()

    if resizing, array should be double the length that it needs, with a minimum


    let src = [1, 2, 3, 4];
    let mut dst = [0, 0];

    // Because the slices have to be the same length,
    // we slice the source slice from four elements
    // to two. It will panic if we don't do this.
    dst.copy_from_slice(&src[2..]);

    assert_eq!(src, [1, 2, 3, 4]);
    assert_eq!(dst, [3, 4]);

    */

    /*
    two major ways to do this
    1. assume that every element in the array up to some place (itemCount), has an associated item in the inventory
    2. don't maintain that assumption
        - implement some sort of searching in either direction from the index drawn
        - ie +1, -1 from the value drawn, until you get something nonzero


    #1 is a bit slower, possibly impossible for large values (not sure)

    either way, u will need something that lets u resize the buffer
    */

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        //itemCount = 10
        //can do swap remove to put a zero there
        let mut indices_array: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let len: u8 = indices_array.len() as u8;

        let draw = 6 as u8;
        let mut indices_vec = to_vec(&indices_array)?;
        indices_vec.remove(draw.into());
        indices_vec.push(0);

        //copy back into an array
        //const new_len: usize = usize::from(len - 1);
        let new_arr: [u8; 10] = indices_vec.try_into().unwrap();
        //msg!("new arr: {:?}", new_arr);

        //indices_array.copy_from_slice(src)
        //dst[..4].clone_from_slice(&src)

        indices_array = new_arr;

        //interesting
        // indices_array[..10].clone_from_slice(&new_arr);
        msg!("indic arr: {:?}", indices_array);

        //hmm
        //so if i can only use 100
        // let mut test_arr = [0 as u8; 2000];
        // let sl = test_arr.as
        /*
        so if u have an array of 10k u will have to put it on the stack in order to shift it
        this means if u have 10k sequential
        u will have to shift over every single one of them manually to evade stack size
        max size u can bring onto the stack with u8s is about 1900
        will be smaller with u16s

        this is a bit of a blow for manual clean up

        options

        if you have a set of arrays that are 1k elements each
        just write an algo that brings them on to the stack one at a time
        and shifts them



        */

        sol_log_compute_units();

        //if i run it exactly like this i would have enough for 13k. that was 20mins of optimizing i def think i could redesign for 30k
        //wasting space with u16s, but not really compute units
        /*
        the constraint is how do u delete the value at index 9 and update all the arrays up to index 10_000
        one option is that u just don't have to do it in one transaction
        one transaction to draw, but u can't draw if the array is too empty
        can use multiple txns to clean up the array
        def an approach that could work
        */
        let mut first = [1 as u16; 600];
        let mut second = [1 as u16; 600];

        //say we want to remove an element at index 9 (bc its being sold)
        //remove the element
        //grab first element in second and put it at the end
        //remove off the front of second
        let mut first_vec: Vec<u16> = first.to_vec();
        first_vec.remove(9);
        first_vec.push(second[0]);
        first = first_vec.try_into().unwrap();

        let mut second_vec = first.to_vec();
        second_vec.remove(0);
        second_vec.push(0);
        second = second_vec.try_into().unwrap();

        let val = second[0];
        let vall = second[599];
        msg!("{}, {}", val, vall);

        // let mut test_vec = to_vec(&test_arr)?;
        // test_vec.remove(50);
        // test_vec.push(0);
        // test_arr = test_vec.try_into().unwrap();
        sol_log_compute_units();
        let mut x = 0;
        for i in 0..50000 {
            x = i;
        }
        sol_log_compute_units();

        panic!();
        Ok(())
    }

    //this is the basics of what u would be doing in the draw
    //and then to resize, u would just close the account, open it back up with more space
}

#[derive(Accounts)]
pub struct Initialize {}
