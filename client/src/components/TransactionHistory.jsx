import React from 'react';

const TransactionHistory = ({ transactions }) => {
  return (
    <div>
      <h2>Transaction History</h2>
      <ul>
        {transactions.map((tx) => (
          <li key={tx}>
            Transaction: {tx}
            {/* Add more transaction details here (e.g., timestamp, tokens swapped, amounts) */}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default TransactionHistory;
