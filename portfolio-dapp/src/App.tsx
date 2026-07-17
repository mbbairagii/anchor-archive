import React, { FC, useEffect, useMemo, useState } from 'react';
import { ConnectionProvider, WalletProvider, useConnection, useWallet } from '@solana/wallet-adapter-react';
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base';
import { UnsafeBurnerWalletAdapter } from '@solana/wallet-adapter-wallets';
import {
  WalletModalProvider,
  WalletDisconnectButton,
  WalletMultiButton
} from '@solana/wallet-adapter-react-ui';
import { clusterApiUrl } from '@solana/web3.js';

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
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  )
}

function Topbar() {
  const {publicKey} = useWallet();
  return <div style={{ display: "flex", justifyContent: "flex-end" }}>
    {!publicKey && <WalletMultiButton />}
    {publicKey && <WalletDisconnectButton />}
  </div>
}


function Portfolio() {
  const { publicKey } = useWallet();
  const {connection} = useConnection();
  const [balance, setBalance] = useState<null | number>(null);

  useEffect(() => {
    if(publicKey) {
      connection.getBalance(publicKey).then((balance) => {
        setBalance(balance);

      });
    }
  }, [publicKey]);
  return <div>
    {publicKey ? <p>Connected wallet: {publicKey.toBase58()}</p> : <p>Please connect your wallet.</p>}
    {balance !== null && <p>Balance: {balance / 1e9} SOL</p>}
  </div>
}

export default App
