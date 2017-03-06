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

const graph : bool = false;
const answer : bool = true;

unsafe fn run() {
    let stdin = io::stdin();
    let mut ilines = stdin.lock().lines();
    let nProblems = ilines.next().unwrap().unwrap().parse::<NodeId>().unwrap();
    if answer {
        println!("Number of problems {}",nProblems);
    }
    for problem in 0..nProblems {
        let nconn = ilines.next().unwrap().unwrap().parse::<usize>().unwrap();
        if answer {
            println!("Problem {}\nnumber of connections {}",problem, nconn);
        }
        if graph {
            //println!("digraph problem{} {{",problem);
            println!("digraph {{");
        }
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
            if graph {
                println!("g{}n{} -> g{}n{};",problem,from,problem,to);
            }
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
                                if answer {
                                    println!("Not a tree! Failed at node {}", neighbor);
                                }
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
        let mut root = 0;
        let mut roots = 0;
        for (i, d) in (&inDegrees).iter().enumerate() {
            if *d == 0 {
                root = i;
                roots+=1;
                if roots > 1 {
                    if answer {
                        println!("Not a tree! Wrong number of roots! ({})", roots);
                    }
                    isTree = false;
                    break;
                }
            }
        }
        if nprocessed != nnodes {
            isTree = false;
            if answer {
                println!("Not a tree, there are unprocessed nodes.");
            }
        }
        if isTree {
            let mut stack : Vec<NodeId> = Vec::new();
            let mut colors : Vec<Color> = iter::repeat(Color::White).take(nnodes).collect();
            stack.push(root);
            colors[root] = Color::Grey;
            while stack.len() > 0 {
                let node = stack.pop().unwrap();
                for neighbor in &fast_graph[node] {
                    match colors[*neighbor] {
                        Color::White => {
                            stack.push(*neighbor);
                            colors[*neighbor] = Color::Grey;
                        }
                        _ => {/*ignore*/}
                    }
                }
                colors[node] = Color::Black;
            }
            match colors.iter().position(|x| match *x {
                Color::White => true,
                _ => false,
            }) {
                Some(x) => {
                    isTree = false;
                    println!("Not a tree! Walk from root {} failed to reach {}!",root,x);
                }
                None => {}
            }
        }
        if isTree {
            if answer {
                println!("Is a tree!");
            }
        }
        if graph {
            println!("}}\n");
        }
    }
}

fn main () {
    unsafe{ run(); }
}
