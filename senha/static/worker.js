self.onmessage = function(e) {
    const passwords = e.data;
    const results = processarEmLotes(passwords, 1000); // Processa as senhas em lotes de 1000
    self.postMessage(results); // Envia os resultados de volta para o main thread
};

// Função para processar as senhas em lotes
function processarEmLotes(passwords, tamanhoLote) {
    const resultados = [];
    for (let i = 0; i < passwords.length; i += tamanhoLote) {
        const lote = passwords.slice(i, i + tamanhoLote);
        const resultadosLote = analisar_senhas(lote); // Chama a função WebAssembly para analisar as senhas
        resultados.push(...resultadosLote);
    }
    return resultados;
}
