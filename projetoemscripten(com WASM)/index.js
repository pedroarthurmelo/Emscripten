// Função que carrega o WebAssembly e utiliza a função `add` exportada
async function loadWasm() {
    // Carrega o arquivo `.wasm`
    const response = await fetch('./pkg/calculadora_bg.wasm');
    const wasmModule = await WebAssembly.instantiateStreaming(response);
    return wasmModule.instance.exports;
}

async function run() {
    // Inicializa o WebAssembly
    const wasmInstance = await loadWasm();

    // Obtém os valores dos inputs
    const a = parseInt(document.getElementById("inputA").value);
    const b = parseInt(document.getElementById("inputB").value);

    // Verifica se os valores são números
    if (isNaN(a) || isNaN(b)) {
        document.getElementById("result").innerText = 'Por favor, insira números válidos.';
        return;
    }

    // Chama a função `add` do WebAssembly
    const result = wasmInstance.add(a, b);
    // Exibe o resultado
    document.getElementById("result").innerText = `Resultado: ${result}`;
}

// Adiciona o Event Listener para o botão
window.addEventListener('DOMContentLoaded', () => {
    document.getElementById("calculateButton").addEventListener('click', run);
});
