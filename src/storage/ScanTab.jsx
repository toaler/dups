import React, {useEffect, useRef, useState} from 'react';
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import DirectionsRunIcon from '@mui/icons-material/DirectionsRun';
import ScanHeader from "./ScanHeader.jsx";
import "./ScanTab.css";

function ScanTab() {
    const ScanStatus = {
        Stopped: "Stopped", Scanning: "Scanning", Completed: "Completed", Failed: "Failed",
    };

    const startTimeRef = useRef(0);
    const endOfLogsRef = useRef(null);
    const inputRef = useRef(null);
    const [path, setPath] = useState('');
    const [logs, setLogs] = useState([]);
    const [resources, setResources] = useState(0);
    const [directories, setDirectories] = useState(0);
    const [files, setFiles] = useState(0);
    const [size, setSize] = useState(0);
    const [elapsedTime, setElapsedTime] = useState(0);
    const [scanStatus, setScanStatus] = useState(ScanStatus.Stopped);
    const [startTime, setStartTime] = useState(0);
    const [timer, setTimer] = useState(null);

    // Focus on the input when component mounts
    useEffect(() => {
        if (inputRef.current) {
            inputRef.current.focus();
        }
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

    useEffect(() => {
        // Function to handle incoming log events
        const handleLogEvent = (event) => {

            console.log(event.payload);

            try {
                const data = JSON.parse(event.payload);
                console.log(data); // Now `data` is a JavaScript object.
                setLogs((currentLogs) => [...currentLogs, event.payload]);

                // Update resources
                setResources((currentResources) => currentResources + data.resources);

                // Update directories
                setDirectories((currentDirectories) => currentDirectories + data.directories);

                // Update files
                setFiles((currentFiles) => currentFiles + data.files);

                setSize((currentSize) => currentSize + data.size);
            } catch (e) {
                console.error(`Error parsing JSON: ${e}`);
            }
        };

        // Start listening for log events from the Rust side
        const unsubscribe = listen("log-event", handleLogEvent);

        // Cleanup the listener when the component unmounts
        return () => {
            unsubscribe.then((unsub) => unsub());
        };
    }, []);


    useEffect(() => {
        endOfLogsRef.current?.scrollIntoView({behavior: 'smooth'});
    }, [logs]); // Dependency array, this effect runs when `logs` changes

    async function scanFilesystem(path) {
        try {
            console.log("Scanning for = " + path);
            setStartTime(Date.now());
            setElapsedTime(0); // Reset elapsed time
            setScanStatus(ScanStatus.Scanning);
            const result = await invoke('scan_filesystem', {path});
            setScanStatus(ScanStatus.Completed);
            console.log(result); // Process result
        } catch (error) {
            setScanStatus(ScanStatus.Failed);
            console.error(error); // Handle error
        }
    }

    const handleScanClick = () => {
        // Reset states
        setResources(0);
        setDirectories(0);
        setFiles(0);

        scanFilesystem(path);
    };


    return (
        <div>
            <input
                ref={inputRef}
                type="text"
                value={path}
                onChange={(e) => setPath(e.target.value)}
                placeholder="Enter filesystem path"
            />
            <button onClick={() => handleScanClick(path)}>
                <DirectionsRunIcon style={{fontSize: 15}}/>
            </button>

            <ScanHeader status={scanStatus} elapsedTime={elapsedTime} resources={resources} directories={directories} files={files} size={size}></ScanHeader>

            <div className="log-container" style={{height: '300px', overflowY: 'auto'}}>
                {logs.map((log, index) => (<div key={index}>{log}</div>))}
                {/* Invisible div at the end of your logs */}
                <div ref={endOfLogsRef}/>
            </div>
        </div>);
}

export default ScanTab;