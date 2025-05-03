import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import { initVeilid, startVeilid } from './veilid/veilid-core.ts'

async function main() {
    await initVeilid();
    await startVeilid();

    createRoot(document.getElementById('root')!).render(
        <StrictMode>
            <App />
        </StrictMode>,
    )
}

main().catch((e) => {
    console.error(e);
});

