import * as wasm from "nn-wasm";
import * as fabric from 'fabric';

const canvas = document.getElementById("drawingCanvas");
const scaledCanvas = document.getElementById("scaledCanvas");
const ctx = canvas.getContext("2d");

ctx.fillStyle = "#ffffff";
ctx.fillRect(0, 0, canvas.width, canvas.height);

const model = await getModel();

async function getModel() {
    const response = await fetch("/assets/my_weights.json");
    
    return await response.json();
}

async function modelPredict() {
    const ctxScaled = scaledCanvas.getContext("2d");
    ctxScaled.save();
    ctxScaled.clearRect(0, 0, ctxScaled.canvas.height, ctxScaled.canvas.width);
    ctxScaled.scale(28.0 / ctx.canvas.width, 28.0 / ctx.canvas.height);
    ctxScaled.drawImage(document.getElementById("drawingCanvas"), 0, 0);
    ctxScaled.restore();
    
    return wasm.predict(ctxScaled.getImageData(0, 0, 28, 28).data, model);
}

document.getElementById("predictBtn").onclick = async () => {
    let preds = await modelPredict();
    
    alert(`Predicted digit: ${preds.indexOf(Math.max(...preds))}`);
}

var newCanvas = new fabric.Canvas("drawingCanvas");
newCanvas.isDrawingMode = true;
newCanvas.freeDrawingBrush = new fabric["PencilBrush"](newCanvas);
newCanvas.freeDrawingBrush.width = 20;
newCanvas.freeDrawingBrush.color = "#000000";
newCanvas.backgroundColor = "#ffffff";
newCanvas.renderAll();