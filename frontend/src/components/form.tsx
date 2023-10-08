import {
  Box,
  Button,
  FormControlLabel,
  FormGroup,
  TextField,
  Tooltip,
} from "@mui/material";
import Checkbox from "@mui/material/Checkbox";
import Grid from "@mui/material/Unstable_Grid2";
import type { SubmitHandler } from "react-hook-form";
import { Controller, useForm } from "react-hook-form";
import toast from "react-hot-toast";
import { convertDar2oar } from "../tauri_cmd";
import ConvertButton from "./convert_btn";
import { PathSelector } from "./path_selector";
import { LogFileButton } from "./log_file_btn";

type FormProps = {
  src: string;
  dist: string;
  modName: string;
  modAuthor: string;
  mappingPath: string;
  mapping1personPath: string;
  loading: boolean;
  runParallel: boolean;
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
      runParallel: localStorage.getItem("runParallel") === "true",
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

  const handleAllClear = () => {
    setStorage("dist")("");
    setStorage("mapping1personPath")("");
    setStorage("mappingPath")("");
    setStorage("modAuthor")("");
    setStorage("modName")("");
    setStorage("src")("");
  };

  const onSubmit: SubmitHandler<FormProps> = async ({
    src,
    dist,
    modName,
    modAuthor,
    mappingPath,
    mapping1personPath,
    runParallel,
  }) => {
    setLoading(true);

    await convertDar2oar({
      src,
      dist,
      modName,
      modAuthor,
      mappingPath,
      mapping1personPath,
      runParallel,
    }).catch((e) => {
      toast.error(`${e}`);
      setLoading(false);
    });
    setLoading(false);
  };

  return (
    <Grid container component="form" onSubmit={handleSubmit(onSubmit)}>
      <FormGroup onSubmit={handleSubmit(onSubmit)}>
        <Button
          sx={{ width: "100%", marginBottom: "50px" }}
          variant="outlined"
          onClick={handleAllClear}
        >
          <span>All Clear</span>
        </Button>

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
            <Grid container spacing={2}>
              <Grid xs={10}>
                <TextField
                  sx={{ width: "100%" }}
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
              </Grid>

              <Grid xs={2}>
                <PathSelector path={value} isDir setValue={setStorage("src")} />
              </Grid>
            </Grid>
          )}
        />

        <Controller
          name="dist"
          control={control}
          render={({
            field: { onChange, onBlur, value },
            fieldState: { error },
          }) => (
            <Grid container spacing={2}>
              <Grid xs={10}>
                <TextField
                  sx={{ width: "100%" }}
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
              </Grid>
              <Grid xs={2}>
                <PathSelector
                  path={value}
                  isDir
                  setValue={setStorage("dist")}
                />
              </Grid>
            </Grid>
          )}
        />

        <Controller
          name="mappingPath"
          control={control}
          render={({
            field: { onChange, onBlur, value },
            fieldState: { error },
          }) => (
            <Grid container spacing={2}>
              <Grid xs={10}>
                <TextField
                  sx={{ width: "100%" }}
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
              </Grid>

              <Grid xs={2}>
                <PathSelector
                  path={value}
                  setValue={setStorage("mappingPath")}
                />
              </Grid>
            </Grid>
          )}
        />

        <Controller
          name="mapping1personPath"
          control={control}
          render={({
            field: { onChange, onBlur, value },
            fieldState: { error },
          }) => (
            <Grid container spacing={2}>
              <Grid xs={10}>
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
              </Grid>

              <Grid xs={2}>
                <PathSelector
                  path={value}
                  setValue={setStorage("mapping1personPath")}
                />
              </Grid>
            </Grid>
          )}
        />

        <Grid container spacing={2}>
          <Grid xs={4}>
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
          </Grid>

          <Grid xs={4}>
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

          <Grid xs={4}>
            <LogFileButton />
          </Grid>

          <Grid xs={4}>
            <Controller
              name="runParallel"
              control={control}
              render={({ field: { value } }) => (
                <Tooltip title="Use multi-threading. (In most cases, it slows down by tens of ms, but may be effective when there is more weight on CPU processing with fewer files to copy and more logic parsing of _condition.txt)">
                  <FormControlLabel
                    control={
                      <Checkbox
                        onClick={() => {
                          localStorage.setItem("runParallel", `${!value}`);
                          setValue("runParallel", !value);
                        }}
                        checked={value}
                        aria-label="Run Parallel"
                      />
                    }
                    label="Run Parallel"
                  />
                </Tooltip>
              )}
            />
          </Grid>
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
