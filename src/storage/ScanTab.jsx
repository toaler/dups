import React, {useEffect, useRef, useState} from 'react';
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import DirectionsRunIcon from '@mui/icons-material/DirectionsRun';
import ScanTabStats from "./ScanTabStats.jsx";
import './ScanTab.css';
import ScanTabLog from "./ScanTabLog.jsx";
import styled from "styled-components";
import {homeDir} from "@tauri-apps/api/path";

function ScanTab() {

    const ScanStatus = {
        Stopped: "Stopped", Scanning: "Scanning", Completed: "Completed", Failed: "Failed",
    };

    const startTimeRef = useRef(0);
    const inputRef = useRef(null);
    const [path, setPath] = useState();
    const [logs, setLogs] = useState([]);
    const [resources, setResources] = useState(0);
    const [directories, setDirectories] = useState(0);
    const [files, setFiles] = useState(0);
    const [size, setSize] = useState(0);
    const [elapsedTime, setElapsedTime] = useState(0);
    const [scanStatus, setScanStatus] = useState(ScanStatus.Stopped);
    const [timer, setTimer] = useState(null);

    useEffect(() => {
        if (inputRef.current) {
            inputRef.current.focus();
        }
    }, []);

    useEffect(() => {
        homeDir().then((dir) => {
            setPath(dir);
        }).catch((error) => {
            console.error('Failed to get home directory', error);
        });
    }, []);

    useEffect(() => {
        let interval = null;

        if (scanStatus === ScanStatus.Scanning && !timer) {
            startTimeRef.current = Date.now();
            interval = setInterval(() => {
                setElapsedTime(oldElapsedTime => Math.floor((Date.now() - startTimeRef.current)));
            }, 100);
            setTimer(interval);
        } else if (scanStatus !== ScanStatus.Scanning && timer) {
            clearInterval(timer);
            setTimer(null);
        }

        return () => {
            if (interval) {
                clearInterval(interval);
            }
        };
    }, [scanStatus]);

    const handleScanClick = () => {
        // Reset states
        setResources(0);
        setDirectories(0);
        setFiles(0);
        setSize(0);
        setLogs([]);

        scanFilesystem(path);
    };

    async function scanFilesystem(path) {
        try {
            startTimeRef.current = Date.now();
            setElapsedTime(0); // Reset elapsed time
            setScanStatus(ScanStatus.Scanning);
            const result = await invoke('scan_filesystem', {path});
            setScanStatus(ScanStatus.Completed);
        } catch (error) {
            setScanStatus(ScanStatus.Failed);
            console.error(error); // Handle error
        }
    }

    const handleLogEvent = (event) => {
        try {
            console.log(event);
            const data = JSON.parse(event.payload);
            setLogs((currentLogs) => [...currentLogs, event.payload]);
            setResources((currentResources) => currentResources + data.resources);
            setDirectories((currentDirectories) => currentDirectories + data.directories);
            setFiles((currentFiles) => currentFiles + data.files);
            setSize((currentSize) => currentSize + data.size);
        } catch (e) {
            console.error(`Error JSON encoded event ${event}, with error ${e}`);
        }
    };

    useEffect(() => {
        const unsubscribe = listen("log-event", handleLogEvent);

        // Cleanup the listener when the component unmounts
        return () => {
            unsubscribe.then((unsub) => unsub());
        };
    }, []);

    return (
        <div>
            <div className="scantab-input">
                <input
                    className="styled-input"
                    type="text"
                    value={path}
                    onChange={(e) => setPath(e.target.value)}
                    placeholder="Enter filesystem path"
                />
                <button className="styled-button" onClick={() => handleScanClick(path)}>
                    <DirectionsRunIcon/>
                </button>
            </div>
            <ScanTabStats status={scanStatus} elapsedTime={elapsedTime} resources={resources} directories={directories}
                          files={files} size={size}></ScanTabStats>
            <ScanTabLog logs={logs}/>
        </div>);
}

export default ScanTab;