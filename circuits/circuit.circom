pragma circom 2.1.6;
// bipartite graph: 2 colors possible { 1,2 }
template Example () {
    // imagine a bipartite graph with a `W` shape connected nodes. bipartite graphs share a special propriety: no two neighboring nodes share the same color.
    // reference img: https://mathonline.wdfiles.com/local--files/bipartite-and-complete-bipartite-graphs/Screen%20Shot%202014-02-09%20at%2011.28.26%20PM.png

    // lets scheme the graph, our `W` shaped graph should have 5 nodes.
    signal input node1; // top left
    signal input node2; // bottom left;
    signal input node3; // middle
    signal input node4; // bottom right
    signal input node5; // top right

    // constraints first. Example: 0 === (1 - color) * (2 - color); // to make sure that the color is either color 1, or 2.
    0 === (1 - node1) * (2 - node1);
    0 === (1 - node2) * (2 - node2);
    0 === (1 - node3) * (2 - node3);
    0 === (1 - node4) * (2 - node4);
    0 === (1 - node5) * (2 - node5);

    // now just need to make sure that 2 connected nodes do not share the same color 
    node1 + node2 === 3; // because color 1 +  color 2 , as they should be different and we only have 2 colors.
    node2 + node3 === 3; // and so on...
    node3 + node4 === 3;
    node4 + node5 === 3;
}

component main { public [node1,
node2,
node3,
node4] } = Example();


/* INPUT = {
    "node1": "1",
    "node2": "2",
    "node3": "1",
    "node4": "2",
    "node5": "1"
} */