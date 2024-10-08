import init, { add } from './pkg/calculadora.js'; // Altere para o nome do seu arquivo .js gerado

        async function run() {
            await init(); // Inicializa o módulo WebAssembly

            // Obtém os valores dos inputs
            const a = parseInt(document.getElementById("inputA").value);
            const b = parseInt(document.getElementById("inputB").value);

            // Verifica se os valores são números
            if (isNaN(a) || isNaN(b)) {
                document.getElementById("result").innerText = 'Por favor, insira números válidos.';
                return;
            }

            // Chama a função add do WebAssembly
            const result = add(a, b);
            // Exibe o resultado
            document.getElementById("result").innerText = `Resultado: ${result}`;
        }

        // Adiciona o Event Listener para o botão
        window.addEventListener('DOMContentLoaded', () => {
            document.getElementById("calculateButton").addEventListener('click', run);
        });