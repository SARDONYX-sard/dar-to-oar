import InputLabel from "@mui/material/InputLabel";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";
import { forwardRef } from "react";
import { UseFormRegister } from "react-hook-form";
import type { LogLevel } from "@/tauri_cmd";

interface IFormValues {
  logLevel: LogLevel;
}

export function selectLogLevel(logLevel: string) {
  switch (logLevel) {
    case "trace":
    case "debug":
    case "info":
    case "warn":
    case "error":
      return logLevel;
    default:
      return "error";
  }
}

export const SelectLogLevel = forwardRef<
  HTMLSelectElement,
  { value: LogLevel } & ReturnType<UseFormRegister<IFormValues>>
>(function SelectLogLevel({ onChange, onBlur, name, value }, ref) {
  return (
    <>
      <InputLabel id="log-level-select-label">Log Level</InputLabel>
      <Select
        name={name}
        ref={ref}
        onChange={(e) => {
          localStorage.setItem(name, e.target.value);
          onChange(e);
        }}
        onBlur={onBlur}
        labelId="log-level-select-label"
        id="log-level-select"
        value={value}
        label="log level"
      >
        <MenuItem value={"trace"}>Trace</MenuItem>
        <MenuItem value={"debug"}>Debug</MenuItem>
        <MenuItem value={"info"}>Info</MenuItem>
        <MenuItem value={"warn"}>Warning</MenuItem>
        <MenuItem value={"error"}>Error</MenuItem>
      </Select>
    </>
  );
});
