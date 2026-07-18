import React, { FC, useEffect, useMemo, useState } from 'react';
import { ConnectionProvider, WalletProvider, useConnection, useWallet } from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { UnsafeBurnerWalletAdapter } from '@solana/wallet-adapter-wallets';

import {
  WalletModalProvider,
  WalletDisconnectButton,
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui';

import {
  PublicKey,
  SystemProgram,
  Transaction,
} from '@solana/web3.js';

// Default styles that can be overridden by your app
import '@solana/wallet-adapter-react-ui/styles.css';

function App() {
  const endpoint = import.meta.env.VITE_HELIUS_RPC_URL;
  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={[]} autoConnect>
        <WalletModalProvider>
          <Topbar />
          <Portfolio />
          <Send/>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  )
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


function Send() {
  const { publicKey, sendTransaction } = useWallet();
  const { connection } = useConnection();
  return <div>
    <input id="address" type="text" placeholder="Recipient address" />
    <input id="amount" type="number" placeholder="Amount (SOL)" />
    <button onClick={async () => {
      const transaction = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: publicKey!,
          toPubkey: new PublicKey((document.getElementById("address") as HTMLInputElement).value),
          lamports: parseFloat((document.getElementById("amount") as HTMLInputElement).value) * 1e9,
        })
      );
      await sendTransaction(transaction, connection);
    }}>Send</button>
  </div>
}

export default App
