import React, { useEffect, useState } from 'react';
import { useConnection } from '@solana/wallet-adapter-react';
import { getMintDecimals } from '@project-serum/serum/lib/market';
import { Market } from '@project-serum/serum';

const PriceInfo = ({ selectedTokenA, selectedTokenB }) => {
  const { connection } = useConnection();
  const [price, setPrice] = useState(null);

  useEffect(() => {
    const fetchPrice = async () => {
      if (selectedTokenA && selectedTokenB && selectedTokenA.marketAddress) {
        const marketAddress = new PublicKey(selectedTokenA.marketAddress); // Use the correct market address
        const market = await Market.load(connection, marketAddress, {}, program.programId);
        const bids = await market.loadBids(connection);
        const asks = await market.loadAsks(connection);

        // Assuming bids and asks are not empty:
        const bestBid = bids.getL2(1)[0][0];
        const bestAsk = asks.getL2(1)[0][0];
        const midPrice = (bestBid + bestAsk) / 2;

        const baseDecimals = await getMintDecimals(connection, market.baseMintAddress);
        const quoteDecimals = await getMintDecimals(connection, market.quoteMintAddress);

        const formattedPrice = midPrice / (10 ** baseDecimals) * (10 ** quoteDecimals);
        setPrice(formattedPrice);
      }
    };

    fetchPrice();
  }, [selectedTokenA, selectedTokenB, connection]);

  return (
    <div>
      <h2>Price Information</h2>
      {price !== null && (
        <p>
          1 {selectedTokenA?.symbol} = {price.toFixed(4)} {selectedTokenB?.symbol}
        </p>
      )}
    </div>
  );
};

export default PriceInfo;
