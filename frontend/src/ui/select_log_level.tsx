import { useState } from "react";
import styles from "./converter.module.css";

export const SelectLogLevel = () => {
  const [selectedLogLevel, setSelectedFruit] = useState("error");
  return (
    <>
      <label className={styles.card}>
        <h2>Log level</h2>
        <select
          name="log level"
          value={selectedLogLevel}
          onChange={(e) => setSelectedFruit(e.target.value)}
        >
          <option value="error">Error</option>
          <option value="trace">Trace</option>
          <option value="debug">Debug</option>
          <option value="info">Info</option>
          <option value="warn">Warning</option>
        </select>
      </label>
    </>
  );
};
