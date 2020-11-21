
// the num-iters input is not relevant for the 1D case
// since this parameter changes how many layers the 2D
// board can grow

document.getElementById("1D").addEventListener("click", _ => {
    document.getElementById("num-iters").style.display = "None";
    document.getElementById("num-iters-label").style.display = "None";
    document.getElementById("num-iters-break").style.display = "None";
});

document.getElementById("2D").addEventListener("click", _ => {
    document.getElementById("num-iters").style.display = "inline-block";
    document.getElementById("num-iters-label").style.display = "block";
    document.getElementById("num-iters-break").style.display = "block";
});