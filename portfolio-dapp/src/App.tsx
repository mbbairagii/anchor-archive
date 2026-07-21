import React, { useEffect, useMemo, useState } from 'react';
import { ConnectionProvider, WalletProvider, useConnection, useWallet } from '@solana/wallet-adapter-react';
import {
  WalletModalProvider,
  WalletDisconnectButton,
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import {
  PublicKey,
  VersionedTransaction,
} from '@solana/web3.js';
import { UnsafeBurnerWalletAdapter } from '@solana/wallet-adapter-wallets';
import '@solana/wallet-adapter-react-ui/styles.css';

function App() {
  const endpoint = import.meta.env.VITE_HELIUS_RPC_URL;
  const wallets = useMemo(() => [new UnsafeBurnerWalletAdapter()], []);

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={wallets} autoConnect>
        <WalletModalProvider>
          <Topbar />
          <Portfolio />
          <Swap />
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}

function Topbar() {
  const { publicKey } = useWallet();
  return <div style={{ display: "flex", justifyContent: "flex-end" }}>
    {!publicKey && <WalletMultiButton />}
    {publicKey && <WalletDisconnectButton />}
  </div>
}


function Portfolio() {
  const { publicKey } = useWallet();
  const { connection } = useConnection();
  const [balance, setBalance] = useState<null | number>(null);

  useEffect(() => {
    if (!publicKey) {
      setBalance(null);
      return;
    }

    connection.getBalance(publicKey).then((balance) => {
      setBalance(balance);
    });
  }, [publicKey, connection]);
  return <div>
    {publicKey ? <p>Connected wallet: {publicKey.toBase58()}</p> : <p>Please connect your wallet.</p>}
    {balance !== null && <p>Balance: {balance / 1e9} SOL</p>}
  </div>
}

function Swap() {
  const { publicKey, signTransaction } = useWallet();
  const [amount, setAmount] = useState("0.01");
  const [loading, setLoading] = useState(false);
  const [quote, setQuote] = useState<any>(null);

  const API_KEY = import.meta.env.VITE_JUP_API_KEY;
  const BASE_URL = "https://api.jup.ag/swap/v2";

  const SOL_MINT = "So11111111111111111111111111111111111111112";
  const USDC_MINT = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

  async function getOrder() {
    if (!publicKey) {
      alert("Connect wallet first");
      return;
    }

    if (!API_KEY) {
      alert("Missing VITE_JUP_API_KEY in your env");
      return;
    }

    try {
      setLoading(true);
      setQuote(null);

      const lamports = Math.floor(Number(amount) * 1e9).toString();

      const params = new URLSearchParams({
        inputMint: SOL_MINT,
        outputMint: USDC_MINT,
        amount: lamports,
        taker: publicKey.toBase58(),
      });

      const res = await fetch(`${BASE_URL}/order?${params.toString()}`, {
        headers: {
          "x-api-key": API_KEY,
        },
      });

      const data = await res.json();

      if (!res.ok) {
        throw new Error(data.error || `Order failed with ${res.status}`);
      }

      if (!data.transaction) {
        console.error("No transaction returned:", data);
        throw new Error(data.errorMessage || "No transaction returned from /order");
      }

      console.log("Order response:", data);
      setQuote(data);
    } catch (err) {
      console.error("Order error:", err);
      alert("Order failed. Check console.");
    } finally {
      setLoading(false);
    }
  }

  async function signAndExecute() {
    if (!publicKey) {
      alert("Connect wallet first");
      return;
    }

    if (!signTransaction) {
      alert("This wallet does not support signTransaction");
      return;
    }

    if (!quote?.transaction || !quote?.requestId) {
      alert("Get an order first");
      return;
    }

    try {
      setLoading(true);

      const txBytes = Uint8Array.from(atob(quote.transaction), (c) => c.charCodeAt(0));
      const transaction = VersionedTransaction.deserialize(txBytes);

      const signedTransaction = await signTransaction(transaction);
      const signedBase64 = btoa(
        String.fromCharCode(...signedTransaction.serialize())
      );

      const res = await fetch(`${BASE_URL}/execute`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-api-key": API_KEY,
        },
        body: JSON.stringify({
          signedTransaction: signedBase64,
          requestId: quote.requestId,
        }),
      });

      const result = await res.json();

      if (!res.ok) {
        throw new Error(result.error || `Execute failed with ${res.status}`);
      }

      console.log("Execute response:", result);

      if (result.status === "Success") {
        alert(`Swap success: ${result.signature}`);
      } else {
        alert(`Swap failed: ${result.error || result.code}`);
      }
    } catch (err) {
      console.error("Execute error:", err);
      alert("Execute failed. Check console.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={{ marginTop: 20 }}>
      <h3>Swap SOL → USDC</h3>

      <input
        type="number"
        placeholder="Amount in SOL"
        value={amount}
        onChange={(e) => setAmount(e.target.value)}
      />

      <button onClick={getOrder} disabled={loading} style={{ marginLeft: 8 }}>
        {loading ? "Loading..." : "Get Order"}
      </button>

      {quote && (
        <div style={{ marginTop: 12 }}>
          <p>Input amount: {Number(quote.inAmount) / 1e9} SOL</p>
          <p>Expected output: {Number(quote.outAmount) / 1e6} USDC</p>
          <p>Router: {quote.router}</p>
          <p>Mode: {quote.mode}</p>

          <button onClick={signAndExecute} disabled={loading}>
            {loading ? "Processing..." : "Sign + Execute"}
          </button>
        </div>
      )}
    </div>
  );
}

export default App
