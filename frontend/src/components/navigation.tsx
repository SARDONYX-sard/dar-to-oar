"use client";

import { useEffect, useState } from "react";
import BottomNavigation from "@mui/material/BottomNavigation";
import BottomNavigationAction from "@mui/material/BottomNavigationAction";
import TransformIcon from "@mui/icons-material/Transform";
import SettingsIcon from "@mui/icons-material/Settings";
import { styled } from "@mui/material";
import { useRouter, usePathname } from "next/navigation";

const Nav = styled(BottomNavigation)(() => {
  return {
    position: "fixed",
    bottom: 0,
    width: "100%",
    ".Mui-selected": {
      color: "#99e4ee",
    },
  };
});

export default function MenuNavigation() {
  const router = useRouter();
  const pathname = usePathname();
  const [value, setValue] = useState(0);

  useEffect(() => {
    if (pathname=== "/") {
      setValue(0);
    } else if (pathname=== "/settings") {
      setValue(1);
    }
  }, [setValue, pathname]);

  return (
    <Nav
      showLabels
      value={value}
      onChange={(_event, newValue) => {
        setValue(newValue);
      }}
    >
      <BottomNavigationAction
        label="Convert"
        icon={<TransformIcon />}
        onClick={() => router.push("/")}
      />
      <BottomNavigationAction
        label="Settings"
        icon={<SettingsIcon />}
        onClick={() => router.push("/settings")}
      />
    </Nav>
  );
}
