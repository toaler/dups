import {useEffect, useRef, useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import "./App.css";
import { Tab, Tabs, TabList, TabPanel } from 'react-tabs';
import 'react-tabs/style/react-tabs.css';
import StorageStagingTab from "./StorageStagingTab.jsx";

function App() {

  const ScanStatus = {
    Stopped: "Stopped",
    Scanning: "Scanning",
    Completed: "Completed",
    Failed: "Failed",
  };


  const endOfLogsRef = useRef(null);

  const [path, setPath] = useState('');
  const [logs, setLogs] = useState([]);
  const [resources, setResources] = useState(0);
  const [directories, setDirectories] = useState(0);
  const [files, setFiles] = useState(0);
  const [selectedRows, setSelectedRows] = useState([]);
  const [topKFiles, setTopKFiles] = useState([]);
  const [size, setSize] = useState(0);
  const [scanStatus, setScanStatus] = useState(ScanStatus.Stopped);
  const [startTime, setStartTime] = useState(0);
  const [elapsedTime, setElapsedTime] = useState(0);
  const [timer, setTimer] = useState(null);

  const formatElapsedTime = () => {
    // Convert elapsed time to hours, minutes, and milliseconds
    const hours = Math.floor(elapsedTime / 3600000); // Total hours
    const minutes = Math.floor((elapsedTime % 3600000) / 60000); // Remaining minutes
    const seconds = Math.floor((elapsedTime % 60000) / 1000); // Convert remainder to seconds
    const milliseconds = elapsedTime % 1000; // Milliseconds are the remainder of elapsed time divided by 1000

    // Format milliseconds to ensure it's always displayed as a three-digit number
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}.${milliseconds.toString().padStart(3, '0')}`;
  };

  const handleCheckboxChange = (event) => {
    console.log(event);
    const value = event.target.value;
    const isChecked = event.target.checked;

    // Update the state based on whether the checkbox was checked or unchecked
    if (isChecked) {
      console.log("checked");
      // Add the row index to the selectedRows state
      setSelectedRows(prev => [...prev, value]);
    } else {
      // Remove the row index from the selectedRows state

      console.log("unchecked");
      setSelectedRows(prev => prev.filter(row => row !== value));
    }
  };




  // Suppose you might update this data dynamically, for example, fetching from an API
  useEffect(() => {
    // Function to handle incoming log events
    const handleTopKEvent = (event) => {

      console.log(event.payload);

      try {
        const data = JSON.parse(event.payload);
        console.log(data); // Now `data` is a JavaScript object.
        setTopKFiles(data);
      } catch (e) {
        console.error(`Error parsing JSON: ${e}`);
      }
    };

    // Start listening for log events from the Rust side
    const unsubscribe = listen("top-k-event", handleTopKEvent);

    // Cleanup the listener when the component unmounts
    return () => {
      unsubscribe.then((unsub) => unsub());
    };
  }, []); // Empty dependency array means this effect runs once after the initial render


  // Effect to scroll to the bottom whenever logs update
  useEffect(() => {
    endOfLogsRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]); // Dependency array, this effect runs when `logs` changes


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
    let interval = null;

    console.log("setting TIMER TIMER TIMER timer = " + Math.floor((Date.now() - startTime)));
    if (scanStatus === ScanStatus.Scanning && !timer) {
      setStartTime(Date.now());
      interval = setInterval(() => {
        setElapsedTime(oldElapsedTime => Math.floor((Date.now() - startTime)));
        console.log("setting TIMER TIMER TIMER timer = " + Math.floor((Date.now() - startTime)));
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

  async function scanFilesystem(path) {
    try {
      console.log("Scanning for = " + path);
      setStartTime(Date.now());
      setElapsedTime(0); // Reset elapsed time
      setScanStatus(ScanStatus.Scanning);
      const result = await invoke('scan_filesystem', { path });
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
 <Tabs forceRenderTabPanel defaultIndex={0}>
    <TabList>
      <Tab>Storage</Tab>
      <Tab>Compute</Tab>
      <Tab>Memory</Tab>
      <Tab>Network</Tab>
    </TabList>
    <TabPanel>
      <Tabs forceRenderTabPanel>
        <TabList>
          <Tab>Scan</Tab>
          <Tab>Inspections</Tab>
          <Tab>Staging</Tab>
        </TabList>
        <TabPanel>
          <p>Scan filesystem</p>
          <table>
            <tr>
              <td>
                <input
                    type="text"
                    value={path}
                    onChange={(e) => setPath(e.target.value)}
                    placeholder="Enter filesystem path"
                />
                <button onClick={() => handleScanClick(path)}>Scan</button>
              </td>
              <td>Status</td>
              <td>{scanStatus} {scanStatus === 'Scanning' ? formatElapsedTime() : ''}</td>
              <td>Resources</td>
              <td>{Number(resources).toLocaleString()}</td>
              <td>Directories</td>
              <td>{Number(directories).toLocaleString()}</td>
              <td>Files</td>
              <td>{Number(files).toLocaleString()}</td>
              <td>Size</td>
              <td>{Number(size).toLocaleString()}</td>
            </tr>
          </table>


          <div className="log-container" style={{height: '300px', overflowY: 'auto'}}>
            {logs.map((log, index) => (
                <div key={index}>{log}</div>
            ))}
            {/* Invisible div at the end of your logs */}
            <div ref={endOfLogsRef}/>
          </div>
        </TabPanel>
        <TabPanel>
          <p>Inspections enable automatic high-level analysis of storage</p>
          <table>
            <thead>
            <tr>
              <th>Stage</th>
              <th>Rank</th>
              <th style={{textAlign: "right"}}>Bytes</th>
              {/* Right-align the header */}
              <th style={{textAlign: "left"}}>Path</th>
            </tr>
            </thead>
            <tbody>
            {topKFiles.map((row, index) => (
                <tr key={index}>
                  <td>
                    <input type="checkbox" value={row.path} onChange={handleCheckboxChange}/>
                  </td>
                  <td>{row.rank}</td>
                  <td style={{textAlign: "right"}}>{Number(row.bytes).toLocaleString("en-US")}</td>
                  {/* Right-align and format the bytes column */}
                  <td style={{textAlign: "left"}}>{row.path}</td>
                </tr>
            ))}
            </tbody>
          </table>
        </TabPanel>
        <TabPanel>
          <StorageStagingTab selectedRows={selectedRows}></StorageStagingTab>
        </TabPanel>
      </Tabs>
    </TabPanel>
   <TabPanel>
     <Tabs forceRenderTabPanel>
       <TabList>
         <Tab>Foo</Tab>
       </TabList>
       <TabPanel>
         <p>bar</p>
         <img src="https://upload.wikimedia.org/wikipedia/en/thumb/2/28/Philip_Fry.png/175px-Philip_Fry.png"
              alt="Philip J. Fry"/>
       </TabPanel>
      </Tabs>
    </TabPanel>
  </Tabs>
  );
}

export default App;