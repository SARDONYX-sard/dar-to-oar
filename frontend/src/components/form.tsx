import ConvertButton from "./convert_btn";
import React from "react";
import toast from "react-hot-toast";
import { Box, Grid, FormGroup, TextField } from "@mui/material";
import { Controller, useForm } from "react-hook-form";
import { PathSelector } from "./path_selector";
import { convertDar2oar } from "../tauri_cmd";
import type { SubmitHandler } from "react-hook-form";

type FormProps = {
  src: string;
  dist: string;
  modName: string;
  modAuthor: string;
  mappingPath: string;
  mapping1personPath: string;
  loading: boolean;
};

export function ConvertForm() {
  const { handleSubmit, control, setValue } = useForm({
    mode: "onBlur",
    criteriaMode: "all",
    shouldFocusError: false,
    defaultValues: {
      src: localStorage.getItem("src") ?? "",
      dist: localStorage.getItem("dist") ?? "",
      modName: localStorage.getItem("modName") ?? "",
      modAuthor: localStorage.getItem("modAuthor") ?? "",
      mappingPath: localStorage.getItem("mappingPath") ?? "",
      mapping1personPath: localStorage.getItem("mapping1personPath") ?? "",
      loading: false as boolean,
    } satisfies FormProps,
  });

  const setStorage = (key: keyof FormProps) => {
    return (value: string) => {
      localStorage.setItem(key, value);
      setValue(key, value);
    };
  };

  const setLoading = (loading: boolean) => {
    setValue("loading", loading);
  };

  const onSubmit: SubmitHandler<FormProps> = async ({
    src,
    dist,
    modName,
    modAuthor,
    mappingPath,
    mapping1personPath,
  }) => {
    setLoading(true);

    await convertDar2oar({
      src,
      dist,
      modName,
      modAuthor,
      mappingPath,
      mapping1personPath,
    }).catch((e) => {
      toast.error(`${e}`);
      setLoading(false);
    });
    setLoading(false);
  };

  return (
    <Grid container component="form" onSubmit={handleSubmit(onSubmit)}>
      <FormGroup onSubmit={handleSubmit(onSubmit)}>
        <Controller
          name="src"
          control={control}
          rules={{
            required: "Need Path",
          }}
          render={({
            field: { onChange, onBlur, value },
            fieldState: { error },
          }) => (
            <>
              <TextField
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
                helperText={
                  "Any path under a dir named DynamicAnimationReplacer can be specified."
                }
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
                label="OAR(dist) Directory"
                placeholder="<MOD NAME>"
                value={value}
                variant="outlined"
                margin="dense"
                onChange={(e) => {
                  localStorage.setItem("dist", e.target.value);
                  onChange(e);
                }}
                onBlur={onBlur}
                error={Boolean(error)}
                helperText={
                  "A dir named meshes/actors/ will be created in the specified directory."
                }
              />
              <PathSelector path={value} isDir setValue={setStorage("dist")} />
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
                sx={{ minWidth: "100%" }}
                label="Mapping Table Path"
                placeholder="./mapping_table.txt"
                value={value}
                variant="outlined"
                margin="dense"
                onChange={(e) => {
                  localStorage.setItem("mappingPath", e.target.value);
                  onChange(e);
                }}
                onBlur={onBlur}
                error={Boolean(error)}
                helperText={
                  "Correspondence path that can change the priority number to your own section name instead of the dir name"
                }
              />
              <PathSelector path={value} setValue={setStorage("mappingPath")} />
            </>
          )}
        />

        <Controller
          name="mapping1personPath"
          control={control}
          render={({
            field: { onChange, onBlur, value },
            fieldState: { error },
          }) => (
            <>
              <TextField
                sx={{ minWidth: "100%" }}
                label="Mapping Table Path(For _1st_person)"
                placeholder="./mapping_table_for_1st_person.txt"
                value={value}
                variant="outlined"
                margin="dense"
                onChange={(e) => {
                  localStorage.setItem("mapping1personPath", e.target.value);
                  onChange(e);
                }}
                onBlur={onBlur}
                error={Boolean(error)}
                helperText={
                  "Correspondence path that can change the priority number to your own section name instead of the dir name"
                }
              />
              <PathSelector
                path={value}
                setValue={setStorage("mapping1personPath")}
              />
            </>
          )}
        />

        <Grid>
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
        </Grid>

        <Controller
          name="loading"
          control={control}
          render={({ field: { value } }) => (
            <Box sx={{ width: "100%", paddingTop: "30px" }}>
              <ConvertButton loading={value} setLoading={setLoading} />
            </Box>
          )}
        />
      </FormGroup>
    </Grid>
  );
}
