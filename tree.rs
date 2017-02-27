#![feature(step_by)]
#![allow(non_snake_case)]
use std::io::prelude::*;
use std::io;
use std::iter;
use std::collections::HashMap;

#[derive(Clone)]
enum Color {
    White,
    Grey,
    Black,
}
type NodeId = usize;
type Graph = Vec<Vec<NodeId>>;

unsafe fn run() {
    let stdin = io::stdin();
    let mut ilines = stdin.lock().lines();
    let nProblems = ilines.next().unwrap().unwrap().parse::<NodeId>().unwrap();
    println!("Number of problems {}",nProblems);
    for problem in 0..nProblems {
        let nconn = ilines.next().unwrap().unwrap().parse::<usize>().unwrap();
        println!("Problem {}\nnumber of connections {}",problem, nconn);
        let mut flat_graph : Vec<NodeId> = Vec::with_capacity(nconn * 2);
        for id in ilines.next().unwrap().unwrap().trim().split(" ") {
            flat_graph.push(id.parse().unwrap());
        }
        let mut sorted = flat_graph.clone();
        sorted.sort();
        sorted.dedup();
        let mut id_map = HashMap::new();
        for (i, id) in sorted.iter().enumerate() {
            //println!("{} renamed {}",id,i);
            id_map.insert(id,i);
        }
        let mut fast_graph : Graph = iter::repeat(Vec::new()).take(sorted.len()).collect();
        for i in 0..nconn {
            let from = *(id_map.get(&flat_graph[ 2*i + 0 ]).unwrap());
            let to   = *(id_map.get(&flat_graph[ 2*i + 1 ]).unwrap());
            //println!("{}->{}",from,to);
            fast_graph[from].push(to);
        }
        let nnodes = sorted.len();
        let mut colors : Vec<Color> = iter::repeat(Color::White).take(nnodes).collect();
        //u8 should prooobably be enough here, but let's be cautious
        let mut inDegrees : Vec<u16> = iter::repeat(0).take(nnodes).collect();
        //let outDegrees = inDegree.clone();
        let ptr = colors.as_mut_ptr();
        let mut isTree = true;
        let mut nprocessed = 0;
        for (node, color) in (&mut colors).iter().enumerate() {
            match *color {
                Color::White => {
                    let mut stack : Vec<NodeId> = Vec::new();
                    stack.push(node);
                    nprocessed+=1;
                    *ptr.offset(node as isize) = Color::Grey;
                    while stack.len()>0 {
                        
                        let node = stack.pop().unwrap();
                        //colors[node] = Color::Black;
                        *ptr.offset(node as isize) = Color::Black;
                        
                        for neighbor in &fast_graph[node] {
                            let neighbor = *neighbor;
                            inDegrees[neighbor]+=1;
                            if inDegrees[neighbor] > 1 {
                                stack.clear();//to break parent loop
                                println!("Not a tree! Failed at node {}", neighbor);
                                isTree = false;
                                break
                            }
                            match *ptr.offset(neighbor as isize) {
                                Color::White => {
                                    *ptr.offset(neighbor as isize) = Color::Grey;
                                    stack.push(neighbor);
                                    nprocessed+=1;
                                    //outDegrees[node]+=1;
                                }
                                _ => {
                                    //pass
                                }
                            }
                        }
                    }
                }
                _ => {
                    //ignore
                }
            }
        }
        let mut roots = 0;
        for d in &inDegrees {
            if *d == 0 {
                roots+=1;
            }
        }
        if roots != 1 {
            println!("Not a tree! Wrong number of roots! ({})", roots);
            isTree = false;
        }
        /*
        for color in colors {
            match color {
                Color::White => {
                    println!("Not a tree, there are unprocessed nodes.");
                    isTree = false;
                    break;
                }
                Color::Grey => {
                    println!("Not a tree, there are unprocessed nodes.");
                    isTree = false;
                    break;
                }
                _ => {
                    //pass
                }
            }
        }
         */
        if nprocessed != nnodes {
            println!("Not a tree, there are unprocessed nodes.");
        } else if isTree {
            println!("Is a tree!");
        }
    }
}

fn main () {
    unsafe{ run(); }
}
