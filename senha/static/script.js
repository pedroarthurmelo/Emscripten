// Importa a função de análise de senhas do WebAssembly
import init, { analyze_passwords } from '../../pkg/password_analyzer.js';

// Função principal que é executada quando a página é carregada
async function run() {
    await init(); // Inicializa o WebAssembly

    // Função que é acionada quando o botão "Analisar Senhas" é clicado
    document.getElementById('analyze').onclick = async () => {
        document.getElementById('loading').style.display = 'block'; // Mostra o carregando
        const passwords = document.getElementById('passwords').value.split('\n'); // Obtém as senhas inseridas pelo usuário
        const results = analyze_passwords(passwords); // Chama a função WebAssembly para analisar as senhas
        document.getElementById('results').innerText = results.join('\n'); // Exibe os resultados na página
        document.getElementById('loading').style.display = 'none'; // Esconde o carregando
    };

    // Função que é acionada quando um arquivo CSV ou JSON é carregado
    document.getElementById('fileInput').addEventListener('change', async (event) => {
        const file = event.target.files[0]; // Obtém o arquivo selecionado pelo usuário
        if (!file) {
            return; // Se não houver arquivo, sai da função
        }

        const reader = new FileReader(); // Cria um leitor de arquivos

        reader.onload = async (e) => {
            const content = e.target.result; // Lê o conteúdo do arquivo
            let passwords = [];

            // Verifica o tipo do arquivo (JSON ou CSV) e faz o parsing dos dados
            if (file.type === 'application/json' || file.name.endsWith('.json')) {
                try {
                    const jsonData = JSON.parse(content); // Faz o parsing do conteúdo JSON
                    if (!Array.isArray(jsonData.passwords)) {
                        throw new Error('O arquivo JSON deve conter um objeto com uma propriedade "passwords" que é um array.');
                    }
                    passwords = jsonData.passwords; // Extrai o array de senhas
                } catch (error) {
                    alert('Erro ao analisar o arquivo JSON: ' + error.message);
                    return;
                }
            } else if (file.type === 'text/csv' || file.name.endsWith('.csv')) {
                const csvRows = content.split('\n'); // Divide o arquivo CSV em linhas
                passwords = csvRows.map(row => row.trim()).filter(row => row.length > 0); // Remove linhas vazias
            } else {
                alert('Formato de arquivo não suportado. Use arquivos JSON ou CSV.');
                return;
            }

            document.getElementById('loading').style.display = 'block'; // Mostra o carregando
            const results = analyze_passwords(passwords); // Chama a função WebAssembly para analisar as senhas
            document.getElementById('results').innerText = results.join('\n'); // Exibe os resultados na página
            document.getElementById('loading').style.display = 'none'; // Esconde o carregando
        };

        reader.readAsText(file); // Lê o arquivo como texto
    });
}

// Chama a função principal ao carregar a página
run();
