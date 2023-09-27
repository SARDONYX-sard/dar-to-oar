"use client";

import styles from "./converter.module.css";
import { Toaster } from "react-hot-toast";
import { useToastLimit } from "../hooks";
import { ConvertForm } from "./form";

export default function Converter() {
  useToastLimit(1);

  return (
    <main className={styles.main}>
      <ConvertForm />
      <Toaster position="bottom-right" reverseOrder={false} />
      {/* <DirSelector title={"DAR(src) Directory"} states={[src, setSrc]} /> */}
    </main>
  );
}
