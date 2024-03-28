import React, {useEffect, useState} from 'react';
import {listen} from "@tauri-apps/api/event";
import FolderZipIcon from '@mui/icons-material/FolderZip';
import CompressIcon from '@mui/icons-material/Compress';
import DeleteIcon from '@mui/icons-material/Delete';
import NotListedLocationIcon from '@mui/icons-material/NotListedLocation';
import TextSnippetIcon from '@mui/icons-material/TextSnippet';

function getMimeTypeIcon(mime_type) {
    if (mime_type.startsWith('text/')) {
        return <TextSnippetIcon />; // Use an appropriate icon for text MIME types
    }

    switch (mime_type) {
        case 'application/octet-stream':
            return <NotListedLocationIcon />;
        case 'image/svg':
            return <ImageIcon />;
        // Add more cases as needed for other mime_types
        default:
            return mime_type; // Default case if no specific icon is needed
    }
}

function InspectionTab({setSelectedRows}) {

    const [topKFiles, setTopKFiles] = useState([]);

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

    const handleCheckboxChange = (event) => {
        console.log(event);
        const value = event.target.value;
        const isChecked = event.target.checked;

        // Update the state based on whether the checkbox was checked or unchecked
        if (isChecked) {
            setSelectedRows(prev => [...prev, value]);
        } else {
            setSelectedRows(prev => prev.filter(row => row !== value));
        }
    };

    return (<div>
        <p>Inspections enable automatic high-level analysis of storage</p>
        <table>
            <thead>
            <tr>
                <th>Stage</th>
                <th style={{textAlign: "left"}}>Action</th>
                <th>Rank</th>
                <th style={{textAlign: "right"}}>Bytes</th>
                <th>MimeType</th>
                <th>Comp</th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Modified</span>
                </th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Accessed</span>
                </th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Modified</span>
                    <span style={{display: "block", textAlign: "right"}}>Days</span>
                </th>
                <th>
                    <span style={{display: "block", textAlign: "right"}}>Last</span>
                    <span style={{display: "block", textAlign: "right"}}>Accessed</span>
                    <span style={{display: "block", textAlign: "right"}}>Days</span>
                </th>
                {/* Right-align the header */}
                <th style={{textAlign: "left"}}>Path</th>
            </tr>
            </thead>
            <tbody>
            {topKFiles.map((row, index) => (<tr key={index}>
                <td>
                    <input type="checkbox" value={row.path} onChange={handleCheckboxChange}/>
                </td>
                <td style={{textAlign: "left"}}><DeleteIcon/>{row.compressible === "1" ? <CompressIcon/> : null}</td>
                <td>{row.rank}</td>
                <td style={{textAlign: "right"}}>{Number(row.bytes).toLocaleString("en-US")}</td>
                <td>{row.compressible === "-1" ? <FolderZipIcon/> : getMimeTypeIcon(row.mime_type)}</td>
                <td style={{textAlign: "right"}}>{row.compressible}</td>
                <td style={{textAlign: "right"}}>{row.modified}</td>
                <td style={{textAlign: "right"}}>{row.accessed}</td>
                <td style={{textAlign: "right"}}>{row.modified_days}</td>
                <td style={{textAlign: "right"}}>{row.accessed_days}</td>
                {/* Right-align and format the bytes column */}
                <td style={{textAlign: "left"}}>{row.path}</td>
            </tr>))}
            </tbody>
        </table>
    </div>);
}

export default InspectionTab;