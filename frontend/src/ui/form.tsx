import { SelectLogLevel } from "./select_log_level";
import ConvertButton from "./convert_btn";
import React from "react";
import { Box, TextField, FormGroup } from "@mui/material";
import { Controller, useForm } from "react-hook-form";
import type { SubmitHandler } from "react-hook-form";
import { convertDar2oar } from "../tauri_cmd";
import toast from "react-hot-toast";
import { PathSelector } from "./path_selector";

type FormProps = {
  src: string;
  dist: string;
  modName: string;
  modAuthor: string;
  mappingPath: string;
  logLevel: "trace" | "debug" | "info" | "warn" | "error";
};

const tryGetLogLevel = (): "trace" | "debug" | "info" | "warn" | "error" => {
  const logLevel = localStorage.getItem("logLevel");
  switch (logLevel) {
    case "trace":
    case "debug":
    case "info":
    case "warn":
      return logLevel;
    default:
      return "error";
  }
};

export function ConvertForm() {
  const { register, handleSubmit, control, setValue } = useForm({
    mode: "onBlur",
    criteriaMode: "all",
    shouldFocusError: false,
    defaultValues: {
      src: localStorage.getItem("src") ?? "",
      dist: localStorage.getItem("dist") ?? "",
      modName: localStorage.getItem("modName") ?? "",
      modAuthor: localStorage.getItem("modAuthor") ?? "",
      mappingPath: localStorage.getItem("mappingPath") ?? "",
      logLevel: tryGetLogLevel(),
    } satisfies FormProps,
  });

  const setStorage = (key: keyof FormProps) => {
    return (value: string) => {
      localStorage.setItem(key, value);
      setValue(key, value);
    };
  };

  const [loading, setLoading] = React.useState(false);
  const onSubmit: SubmitHandler<FormProps> = ({
    src,
    dist,
    modName,
    modAuthor,
    mappingPath,
  }) => {
    setLoading(true);
    convertDar2oar({ src, dist, modName, modAuthor, mappingPath }).catch(
      (e) => {
        toast.error(`${e}`);
        setLoading(false);
      }
    );
    setLoading(false);
  };

  return (
    <>
      <Box
        component="form"
        width="100%"
        display="flex"
        flexDirection="column"
        justifyContent="center"
        onSubmit={handleSubmit(onSubmit)}
      >
        <FormGroup onSubmit={handleSubmit(onSubmit)}>
          <Controller
            name="src"
            control={control}
            rules={{
              required: "need PATH",
            }}
            render={({
              field: { onChange, onBlur, value },
              fieldState: { error },
            }) => (
              <>
                <TextField
                  sx={{ minWidth: "80vw" }}
                  label="DAR(src) Directory"
                  placeholder="<MOD NAME>/DynamicAnimationReplacer/_CustomCondition/"
                  required
                  value={value}
                  variant="outlined"
                  margin="dense"
                  onChange={(e) => {
                    localStorage.setItem("src", e.target.value);
                    onChange(e);
                  }}
                  onBlur={onBlur}
                  error={Boolean(error)}
                  helperText={error?.message}
                />

                <PathSelector path={value} isDir setValue={setStorage("src")} />
              </>
            )}
          />

          <Controller
            name="dist"
            control={control}
            render={({
              field: { onChange, onBlur, value },
              fieldState: { error },
            }) => (
              <>
                <TextField
                  sx={{ minWidth: "80vw" }}
                  label="OAR(dist) Directory"
                  placeholder="<MOD NAME>/OpenAnimationReplacer/_CustomCondition/"
                  value={value}
                  variant="outlined"
                  margin="dense"
                  onChange={(e) => {
                    localStorage.setItem("dist", e.target.value);
                    onChange(e);
                  }}
                  onBlur={onBlur}
                  error={Boolean(error)}
                  helperText={error?.message}
                />
                <PathSelector
                  path={value}
                  isDir
                  setValue={setStorage("dist")}
                />
              </>
            )}
          />

          <Controller
            name="mappingPath"
            control={control}
            render={({
              field: { onChange, onBlur, value },
              fieldState: { error },
            }) => (
              <>
                <TextField
                  sx={{ minWidth: "80vw" }}
                  label="Mapping Table Path"
                  placeholder="../mapping_table.txt"
                  value={value}
                  variant="outlined"
                  margin="dense"
                  onChange={(e) => {
                    localStorage.setItem("mappingPath", e.target.value);
                    onChange(e);
                  }}
                  onBlur={onBlur}
                  error={Boolean(error)}
                  helperText={error?.message}
                />
                <PathSelector
                  path={value}
                  setValue={setStorage("mappingPath")}
                />
              </>
            )}
          />

          <Controller
            name="modName"
            control={control}
            render={({
              field: { onChange, onBlur, value },
              fieldState: { error },
            }) => (
              <TextField
                label="Mod Name"
                placeholder="Mod Name"
                value={value}
                variant="outlined"
                margin="dense"
                onChange={(e) => {
                  localStorage.setItem("modName", e.target.value);
                  onChange(e);
                }}
                onBlur={onBlur}
                error={Boolean(error)}
                helperText={error?.message}
              />
            )}
          />

          <Controller
            name="modAuthor"
            control={control}
            render={({
              field: { onChange, onBlur, value },
              fieldState: { error },
            }) => (
              <TextField
                label="Mod Author Name"
                placeholder="Name"
                value={value}
                variant="outlined"
                margin="dense"
                onChange={(e) => {
                  localStorage.setItem("modAuthor", e.target.value);
                  onChange(e);
                }}
                onBlur={onBlur}
                error={Boolean(error)}
                helperText={error?.message}
              />
            )}
          />

          <Controller
            name="logLevel"
            control={control}
            render={({ field: { value } }) => (
              <SelectLogLevel value={value} {...register("logLevel")} />
            )}
          />

          <ConvertButton loading={loading} setLoading={setLoading} />
        </FormGroup>
      </Box>
    </>
  );
}
